// 16 bit computer
#[derive(Copy, Clone)]
struct Register(u16);

struct RegisterParseError;

// common registers
static mut REGISTERS: [Register; 8] = [
    Register(0x0010),
    Register(0x0100),
    Register(0x0201), // 513
    Register(0x01C8), // 456
    Register(0x0000),
    Register(0x0000),
    Register(0x0000),
    Register(0x0000)
];

// input register, can be used to input constant values into registers, and the instructions
// can also use this to store temp value for their operations
static mut IN_REGISTER: Register = Register(0x0000);
// output register, used for debugging, I want to use it for viewing output of the ops
// can only be used as output register
static mut OUT_REGISTER: Register = Register(0x0000);

// TODO formally define the instruction set and machine code
// "add" just adds, nothing new
// use "-" for input register (not done), and "_" for `OUT_REGISTER`
// print, used to ask Rust to print the value of `OUT_REGISTER`
// refer to normal registers with "r" + the register number i.e. r3 is the fourth register
const PROGRAM: &str = "\
add r3, r3, r4
lt
print\
";

fn get_register(reg: &str) -> Result<*mut Register, std::num::ParseIntError> {
    unsafe {
        if reg == "_" {
            return Ok(&mut OUT_REGISTER);
        }
        if reg == "-" {
            return Ok(&mut IN_REGISTER);
        }

        let i = reg[1..].parse::<usize>()?;
        Ok(&mut REGISTERS[i])
    }
}

enum Operation {
    ADD(String, String), // add the value from these to registers and leave it into IN_REGISTER
    MOVE(String, String) // move value from one register to another
}

fn get_value(reg: &str) -> u16 {
    unsafe { (*get_register(reg).expect("Invalid register")).0 }
}

fn set_value(reg: &str, val: u16) {
    unsafe { (*get_register(reg).expect("Invalid register")).0 = val; }
}

fn parse_instruction(ins: &str) -> Vec<Operation> {
    let mut ops = Vec::new();

    if ins.starts_with("add") {
        // WARNING since all registers are one digit, making the assumption that
        // I need to parse between 4..6 and 8..10
        let x = ins[4..6].to_string();
        let y = ins[8..10].to_string();
        ops.push(Operation::ADD(x, y));
        let x = "-".to_string();
        let y = ins[12..].to_string();
        ops.push(Operation::MOVE(x, y));
    }
    ops
}

// panic in case of overflows and stuff
// some stuff may be needed to controlled manually
fn execute_operation(op: Operation) {
    unsafe {
        match op {
            Operation::ADD(x, y) => set_value("-", get_value(x.as_str()) + get_value(y.as_str())),
            Operation::MOVE(x, mut y) => set_value(y.as_str(), get_value(x.as_str()))
        }
    }
}

fn main() {

}

#[cfg(test)]
mod operation_tests {
    use super::*;

    #[test]
    fn it_adds() {
        let ops = parse_instruction("add r0, r1, _");
        ops.into_iter().for_each(execute_operation);  // counting because want to consume
        unsafe {
            assert_eq!(OUT_REGISTER.0, 0x0110);
        }
    }

//    #[test]
//    fn finds_gcd() {
//        let mut PC = 0;
//        for line in PROGRAM {
//            let ops = parse_instruction(line);
//            ops.into_iter().for_each(execute_operation);
//        }
//        unsafe {
//            assert_eq!(OUT_REGISTER.0, 57);
//        }
//    }
}
