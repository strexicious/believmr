use pom::parser::Parser;
use pom::parser::*;
use pom::char_class::*;

use super::engine::{Instruction, Mov, Alu, Jump};

fn whitespace<'a>() -> Parser<'a, u8, ()> {
    is_a(space).repeat(1..).discard()
}

fn whitespace_opt<'a>() -> Parser<'a, u8, ()> {
    is_a(space).repeat(0..).discard()
}

fn dec_value<'a>() -> Parser<'a, u8, String> {
    let value = sym(b'-').opt() + is_a(digit).repeat(1..);
    value.collect().convert(std::str::from_utf8).map(String::from)
}

fn hex_value<'a>() -> Parser<'a, u8, String> {
    let value = seq(b"0x") * is_a(hex_digit).repeat(1..);
    value.convert(String::from_utf8)
}

fn mem_addr<'a>() -> Parser<'a, u8, u16> {
    hex_value().convert(|h| u16::from_str_radix(&h, 16))
}

fn offset<'a>() -> Parser<'a, u8, i16> {
    dec_value().convert(|d| i16::from_str_radix(&d, 10))
}

fn integer<'a>() -> Parser<'a, u8, i32> {
    let dec_repr = dec_value().convert(|d| d.parse());
    let hex_repr = hex_value().convert(|h| u32::from_str_radix(&h, 16)).map(|num| num as i32);
    hex_repr | dec_repr
}

fn alu_op<'a>() -> Parser<'a, u8, u8> {
    let add = seq(b"add").map(|_| 0x11);
    let sub = seq(b"sub").map(|_| 0x12);
    let and = seq(b"and").map(|_| 0x13);
    let or = seq(b"or").map(|_| 0x14);
    let xor = seq(b"xor").map(|_| 0x15);
    let cmp = seq(b"cmp").map(|_| 0x16);
    let sll = seq(b"sll").map(|_| 0x17);
    let srl = seq(b"srl").map(|_| 0x18);
    let sra = seq(b"sra").map(|_| 0x19);
    add | sub | and | or | xor | cmp | sll | srl | sra
}

fn j_cond<'a>() -> Parser<'a, u8, u8> {
    let j = seq(b"j").map(|_| 0x20);
    let jl = seq(b"jl").map(|_| 0x21);
    let jle = seq(b"jle").map(|_| 0x22);
    let je = seq(b"je").map(|_| 0x23);
    jle | jl | je | j
}

fn mov_instr<'a>() -> Parser<'a, u8, Box<dyn Instruction>> {
    let op_code = seq(b"mov").map(|_| 0x10) - whitespace();
    let literal = integer() - whitespace_opt();
    let dest = sym(b',').discard() * whitespace() * mem_addr();
    (op_code + literal + dest).map(|((a, b), c)| Box::new(Mov::new(b, c)) as Box<Instruction>)
}

fn alu_instr<'a>() -> Parser<'a, u8, Box<dyn Instruction>> {
    let op_code = alu_op() - whitespace();
    let src = mem_addr() - whitespace_opt();
    let dest = sym(b',').discard() * whitespace() * mem_addr();
    (op_code + src + dest).map(|((a, b), c)| Box::new(Alu::new(a, b, c)) as Box<Instruction>)
}

fn jmp_instr<'a>() -> Parser<'a, u8, Box<dyn Instruction>> {
    let op_code = j_cond() - whitespace();
    let off = offset();
    (op_code + off).map(|(a, b)| Box::new(Jump::new(a, b)) as Box<Instruction>)
}

pub fn instr<'a>() -> Parser<'a, u8, Box<dyn Instruction>> {
    whitespace_opt() * (mov_instr() | alu_instr() | jmp_instr()) - (whitespace_opt() + end().opt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespaces() {
        assert_eq!(whitespace_opt().parse(b""), Ok(()));
        assert_eq!(whitespace_opt().parse(b" "), Ok(()));
        assert_eq!(whitespace().parse(b" "), Ok(()));
        assert_eq!(whitespace().parse(b"    "), Ok(()));
    }

    #[test]
    fn numbers() {
        assert_eq!(dec_value().parse(b"4352"), Ok(String::from("4352")));
        assert_eq!(dec_value().parse(b"-43522141421421"), Ok(String::from("-43522141421421")));
        assert_eq!(hex_value().parse(b"0xfed0023eabbc334"), Ok(String::from("fed0023eabbc334")));
        assert_eq!(integer().parse(b"0xFFFFFFFF"), Ok(-1i32));
        assert_eq!(integer().parse(b"210491581"), Ok(210_491_581i32));
    }

    #[test]
    fn mem_addr_parses() {
        assert_eq!(mem_addr().parse(b"0xA12B"), Ok(41_259u16));
        assert_eq!(mem_addr().parse(b"0xFFFF"), Ok(65_535u16));
    }

    #[test]
    fn offset_parses() {
        assert_eq!(offset().parse(b"32767"), Ok(32_767i16));
        assert_eq!(offset().parse(b"-32768"), Ok(-32_768i16));
        assert_eq!(offset().parse(b"-568"), Ok(-568i16));
    }
}
