use super::Context;

#[derive(Debug)]
pub enum DecodeError {
    NoOpcode,
    InvalidMov,
    InvalidAlu,
    InvalidJump,
    UnknownOpcode(u8),
}

#[derive(Debug)]
enum SysCall {
    Print(u16),
}

#[derive(Debug)]
pub enum Instruction {
    Mov { literal: i32, dest: u16 },
    Alu { alu_op: u8, src: u16, dest: u16 },
    Jump { jmp_op: u8, offset: i16 },
    SysCall(SysCall),
}

impl Instruction {
    pub fn execute(&self, ctx: &mut Context) -> Result<(), String> {
        use Instruction::*;
        
        match self {
            Mov { literal, dest } => Self::execute_mov(ctx, *literal, *dest),
            Alu { alu_op, src, dest } => Self::execute_alu(ctx, *alu_op, *src, *dest),
            Jump { jmp_op, offset } => Self::execute_jmp(ctx, *jmp_op, *offset),
            SysCall(syscall) => Self::execute_syscall(ctx, syscall),
        }
    }
    
    fn execute_mov(ctx: &mut Context, literal: i32, dest: u16) -> Result<(), String> {
        ctx.mem_block[dest as usize] = literal;
        Ok(())
    }
    
    fn execute_alu(ctx: &mut Context, alu_op: u8, src: u16, dest: u16)
        -> Result<(), String> {
        let src = ctx.mem_block[src as usize];
        let dest = &mut ctx.mem_block[dest as usize];

        match alu_op {
            0x11 => *dest += src,
            0x12 => *dest -= src,
            0x13 => *dest &= src,
            0x14 => *dest |= src,
            0x15 => *dest ^= src,
            0x16 => {
                let cmp = *dest - src;
                if cmp < 0 {
                    ctx.status_register |= 1 << 1;
                    ctx.status_register &= 0xFE;
                } else if cmp == 0 {
                    ctx.status_register |= 1;
                    ctx.status_register &= 0xFD;
                } else {
                    ctx.status_register &= 0xFC;
                }
            },
            0x17 => *dest <<= src as u32,
            0x18 => *dest = ((*dest as u32) >> (src as u32)) as i32,
            0x19 => *dest >>= src as u32,
            0x1A => {
                let res = i64::from(*dest) * i64::from(src);
                *dest = res as i32;
                ctx.special_registers.0 = (res >> 32) as i32;
            },
            0x1B => {
                let res = *dest / src;
                ctx.special_registers.0 = *dest % src;
                *dest = res;
            },
            _ => return Err(format!("Invalid ALU operation: {}", alu_op)),
        }
        Ok(())
    }
    
    fn execute_jmp(ctx: &mut Context, jmp_op: u8, offset: i16) -> Result<(), String> {
        if offset < 0 && ctx.line_counter < (offset.abs() as usize) {
            return Err(String::from("invalid negative offset inside program"));
        }
        if offset > 0 && (ctx.program_size - ctx.line_counter) < (offset as usize) {
            return Err(String::from("invalid positive offset inside program"));
        }

        let new_line = ((ctx.line_counter as isize) + (offset as isize)) as usize;
        let zero_flag_set = (ctx.status_register & 1) == 0x01;
        let undeflow_flag_set = (ctx.status_register & (1 << 1)) == 0x02;
        match jmp_op {
            0x20 => ctx.line_counter = new_line,
            0x21 => if undeflow_flag_set { ctx.line_counter = new_line },
            0x22 => if undeflow_flag_set || zero_flag_set { ctx.line_counter = new_line },
            0x23 => if zero_flag_set { ctx.line_counter = new_line },
            _ => return Err(format!("Invalid jump operation: {}", jmp_op)),
        }
        Ok(())
    }

    fn execute_syscall(ctx: &mut Context, syscall: &SysCall) -> Result<(), String> {
        use SysCall::*;

        match syscall {
            Print(addr) => {
                // WARNING: imperative code ahead, due to flat_map not working nicely with
                // temporary array from to_be_bytes
                let mut unicode_str = Vec::new();
                for sep_bytes in ctx.mem_block[*addr as usize..].iter().map(|dw| dw.to_be_bytes()) {
                    for to_print_char in sep_bytes.iter().take_while(|c| **c as char != '\0') {
                        unicode_str.push(*to_print_char);
                    }
                }

                println!("{}", String::from_utf8_lossy(unicode_str.as_slice()));
                Ok(())
            },
            _ => Err(String::from("Unimplemented syscall")),
        }
    }

    // returns an instruction and rest of bytes
    pub fn decode_instr(bytes: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        use Instruction::*;
        
        if bytes.is_empty() {
            return Err(DecodeError::NoOpcode);
        }

        match bytes[0] {
            0x10 => {
                if bytes.len() < 7 {
                    Err(DecodeError::InvalidMov)
                } else {
                    let literal = i32::from_be_bytes([bytes[1], bytes[2], bytes[3], bytes[4]]);
                    let dest = u16::from_be_bytes([bytes[5], bytes[6]]);
                    Ok((Mov { literal, dest }, &bytes[7..]))
                }
            },
            0x11...0x19 => {
                if bytes.len() < 5 {
                    Err(DecodeError::InvalidAlu)
                } else {
                    let src = u16::from_be_bytes([bytes[1], bytes[2]]);
                    let dest = u16::from_be_bytes([bytes[3], bytes[4]]);
                    Ok((Alu { alu_op: bytes[0], src, dest }, &bytes[5..]))
                }
            },
            0x20...0x23 => {
                if bytes.len() < 3 {
                    Err(DecodeError::InvalidJump)
                } else {
                    let offset = i16::from_be_bytes([bytes[1], bytes[2]]);
                    Ok((Jump { jmp_op: bytes[0], offset }, &bytes[3..]))
                }
            },
            _ => Err(DecodeError::UnknownOpcode(bytes[0])),
        }
    }
}
