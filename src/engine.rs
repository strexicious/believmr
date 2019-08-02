mod instr;

use instr::{Instruction, SysCallCode, DecodeError};

type Memory = [i32; 65536];
type SpecialRegisterBlock = (i32, i32);

pub struct Context {
    mem_block: Memory,
    line_counter: usize,
    status_register: u8,
    special_registers: SpecialRegisterBlock,
    program_size: usize,
    // lookup table: index -> (instr line, # of params)
    subroutine_map: Vec<(usize, u16)>,
    // stack storing (params, mem_block_start) for subroutines 
    subroutine_stack: Vec<(Vec<i32>, u16)>,
}

impl Context {
    fn new(program_size: usize, subroutine_map: Vec<(usize, u16)>) -> Self {
        Self {
            mem_block: [0; 65536],
            line_counter: 0,
            status_register: 0,
            special_registers: (0, 0),
            program_size,
            subroutine_map,
            subroutine_stack: Vec::new(),
        }
    }
}

pub struct Process {
    context: Context,
    program: Vec<Instruction>,
}

impl Process {
    pub fn new(source: Vec<u8>) -> Result<Self, DecodeError> {
        let mut source = source.as_slice();
        let mut subroutine_map = Vec::new();
        let mut program = Vec::new();
        
        while !source.is_empty() {
            let (instr, rest) = Instruction::decode_instr(source)?;
            
            if let Instruction::SysCall(SysCallCode::Subroutine(params)) = instr {
                let (iinstr, irest) = Instruction::decode_instr(source).map_err(|e| {
                    match e {
                        DecodeError::NoOpcode => DecodeError::EmptySubroutine,
                        _ => e,
                    }
                })?;
                
                match iinstr {
                    Instruction::SysCall(SysCallCode::Subroutine(_)) =>
                        return Err(DecodeError::EmptySubroutine),
                    _ => {
                        subroutine_map.push((program.len(), params));
                        program.push(iinstr);
                        source = irest;
                    },
                }
            } else {
                program.push(instr);
                source = rest;
            }
        }
        
        Ok(Self { context: Context::new(program.len(), subroutine_map), program })
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

    pub fn print_program(&self) {
        println!("{:#?}", self.program);
    }
}
