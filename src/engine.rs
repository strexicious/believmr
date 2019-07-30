mod instr;

use instr::Instruction;

type Memory = [i32; 65536];

pub struct Context {
    mem_block: Memory,
    line_counter: usize,
    status_register: u8,
    program_size: usize,
}

impl Context {
    pub fn new(program_size: usize) -> Self {
        Self { mem_block: [0; 65536], line_counter: 0, status_register: 0, program_size }
    }

    pub fn print_mem(&self, pos: u16, offset: u16) {
        println!("{:?}", &self.mem_block[pos as usize..(pos + offset) as usize]);
    }
}

pub struct Process {
    context: Context,
    program: Vec<Instruction>,
}

impl Process {
    pub fn new(source: Vec<u8>) -> Self {
        let mut source = source.as_slice();
        let mut program = Vec::new();
        while source.len() > 0 {
            let (instr, rest) = Instruction::decode_instr(source).unwrap();
            program.push(instr);
            source = rest;
        }
        
        Self { context: Context::new(program.len()), program }
    }

    pub fn run(&mut self) {
        while self.context.line_counter < self.context.program_size {
            self.context.line_counter += 1;
            self.program[self.context.line_counter - 1]
                .execute(&mut self.context)
                .unwrap();
        }
    }
}
