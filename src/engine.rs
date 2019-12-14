pub mod instr;

use instr::{Instruction, DecodeError};

type Memory = [i32; 65536];
type SpecialRegisterBlock = (i32, );

pub enum SysCallInvoc {
    Print(String),
}

struct Context {
    mem_block: Memory,
    line_counter: usize,
    status_register: u8,
    special_registers: SpecialRegisterBlock,
    syscall_invoc: Option<SysCallInvoc>,
}

impl Context {
    fn new() -> Self {
        Self {
            mem_block: [0; 65536],
            line_counter: 0,
            status_register: 0,
            special_registers: (0, ),
            syscall_invoc: None,
        }
    }
}

pub struct Process {
    context: Context,
    program: Vec<Instruction>,
    syscall_handler: fn(&SysCallInvoc),
}

impl Process {
    pub fn new(source: Vec<u8>, syscall_handler: fn(&SysCallInvoc)) -> Result<Self, DecodeError> {
        let mut source = source.as_slice();
        let mut program = Vec::new();
        
        while !source.is_empty() {
            let (instr, rest) = Instruction::decode_instr(source)?;
            
            program.push(instr);
            source = rest;
        }
        
        Ok(Self {
            context: Context::new(),
            program,
            syscall_handler,
        })
    }

    pub fn run(&mut self) {
        while self.context.line_counter < self.program.len() {
            if let Some(syscall_invoc) = self.context.syscall_invoc.take() {
                (self.syscall_handler)(&syscall_invoc);
            }
            
            self.context.line_counter += 1;
            self.program[self.context.line_counter - 1]
                .execute(&mut self.context)
                .unwrap();
        }
    }

    pub fn print_mem(&self, pos: u16, offset: u16) {
        println!("{:?}", &self.context.mem_block[pos as usize..(pos + offset) as usize]);
    }

    pub fn print_program(&self) {
        println!("{:#?}", self.program);
    }
}
