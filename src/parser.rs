use nom::*;

fn parse_u16_hex(input: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(input, 16)
}

fn parse_i32_hex_unsigned(input: &str) -> Result<i32, std::num::ParseIntError> {
    Ok(u32::from_str_radix(input, 16)? as i32)
}

named!(hex_string<&str, &str>, take_while1!(|c: char| c.is_ascii_hexdigit()));
named!(hex_number<&str, &str>, preceded!(tag!("0x"), hex_string));
named!(dec_string<&str, &str>, take_while1!(|c: char| c.is_ascii_digit()));
named!(dec_number<&str, &str>, map!(pair!(opt!(tag!("-")), dec_string),
    move |(s1, s2)| {
        match s1 {
            Some(s1) => {
                let l1 = s1.len();
                let l2 = s2.len();
                let s1_ptr = s1.as_ptr();

                unsafe {
                    let joined = std::slice::from_raw_parts(s1_ptr, l1 + l2);
                    std::str::from_utf8_unchecked(joined)
                }
            },
            None => s2
        }
    }));

named!(mem_addr<&str, u16>, map_res!(hex_number, parse_u16_hex));
named!(offset<&str, i16>, flat_map!(dec_number, parse_to!(i16)));
named!(literal<&str, i32>,
    alt!(
        flat_map!(dec_number, parse_to!(i32)) |
        map_res!(hex_number, parse_i32_hex_unsigned)
    )
);

named!(mov_instr<&str, [u8; 7]>, do_parse!(
    tag!("mov ") >>
    op_literal: literal >>
    tag!(", ") >>
    op_dest: mem_addr >>
    ({
        let lit_bytes = op_literal.to_be_bytes();
        let dest_bytes = op_dest.to_be_bytes();
        [0x10, lit_bytes[0], lit_bytes[1], lit_bytes[2], lit_bytes[3], dest_bytes[0], dest_bytes[1]]
    })
));

fn create_alu_parser<'a>(mnemonic: &'a str, opcode: u8) -> impl Fn(&str) -> IResult<&str, [u8; 5]> + 'a
{
    move |input: &str| do_parse!(input,
        tag!(mnemonic) >>
        tag!(" ") >>
        src: mem_addr >>
        tag!(", ") >>
        dest: mem_addr >>
        ({
            let src_bytes = src.to_be_bytes();
            let dest_bytes = dest.to_be_bytes();
            [opcode, src_bytes[0], src_bytes[1], dest_bytes[0], dest_bytes[1]]
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    
    #[test]
    fn mem_addr_parses() {
        assert_eq!(Ok(("", 4847)), mem_addr("0x12EF"));
        assert_eq!(Err(Err::Error(("12EF", ErrorKind::Tag))), mem_addr("12EF"));
        assert_eq!(Err(Err::Incomplete(Needed::Size(2))), mem_addr("0x12"));
    }

    #[test]
    fn offset_parses() {
        assert_eq!(Ok(("", 32767)), offset("32767"));
        assert_eq!(Ok(("", -32768)), offset("-32768"));
        assert_eq!(Err(nom::Err::Error(("32768", ErrorKind::ParseTo))), offset("32768"));
    }

    #[test]
    fn literal_parses() {
        assert_eq!(Ok(("", -1_635_123)), literal("-1635123"));
        assert_eq!(Ok(("", -1_043_075_071)), literal("0xC1D3F001"));
        assert_eq!(Err(nom::Err::Error(("Wot", ErrorKind::Alt))), literal("Wot"));
    }

    #[test]
    fn mov_parses() {
        assert_eq!(
            Ok(("", [0x10, 0, 0, 0x13, 0x37, 0x23, 0])),
            mov_instr("mov 0x00001337, 0x2300"));
        assert_eq!(
            Ok(("", [0x10, 0xFF, 0xFF, 0xFF, 0xFE, 0xFE, 0xED])),
            mov_instr("mov -2, 0xFEED"));
    }
}
