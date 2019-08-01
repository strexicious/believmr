mod instr;

use instr::Instruction;

type Memory = [i32; 65536];
type SpecialRegisterBlock = (i32, u16, u16);

pub struct Context {
    mem_block: Memory,
    line_counter: usize,
    status_register: u8,
    special_registers: SpecialRegisterBlock,
    program_size: usize,
}

impl Context {
    fn new(program_size: usize) -> Self {
        Self {
            mem_block: [0; 65536],
            line_counter: 0,
            status_register: 0,
            special_registers: (0, 0, 0),
            program_size
        }
    }
}

pub struct Process {
    context: Context,
    subroutines: Vec<u16>,
    program: Vec<Instruction>,
}

impl Process {
    pub fn new(source: Vec<u8>) -> Self {
        let mut source = source.as_slice();
        let mut program = Vec::new();
        
        while !source.is_empty() {
            let (instr, rest) = Instruction::decode_instr(source).unwrap();
            program.push(instr);
            source = rest;
        }
        
        Self { context: Context::new(program.len()), subroutines: Vec::new(), program }
    }

    pub fn run(&mut self) {
        while self.context.line_counter < self.context.program_size {
            self.context.line_counter += 1;
            self.program[self.context.line_counter - 1]
                .execute(&mut self.context)
                .unwrap();
        }
    }

    pub fn print_mem(&self, pos: u16, offset: u16) {
        println!("{:?}", &self.context.mem_block[pos as usize..(pos + offset) as usize]);
    }
}
