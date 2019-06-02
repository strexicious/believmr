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

    pub fn print_mem(&self, pos: u16) {
        println!("{}", self.mem_block[pos as usize]);
    }
}

pub trait Instruction {
    fn execute(&self, ctx: &mut Context) -> Result<(), String>;
}

pub struct Mov {
    literal: i32,
    dest: u16,
}

impl Mov {
    pub fn new(literal: i32, dest: u16) -> Self {
        Self { literal, dest }
    }
}

impl Instruction for Mov {
    fn execute(&self, ctx: &mut Context) -> Result<(), String> {
        ctx.mem_block[self.dest as usize] = self.literal;
        Ok(())
    }
}

pub struct Alu {
    alu_op: u8,
    src: u16,
    dest: u16,
}

impl Alu {
    pub fn new(alu_op: u8, src: u16, dest: u16) -> Self {
        Self { alu_op, src, dest }
    }
}

impl Instruction for Alu {
    fn execute(&self, ctx: &mut Context) -> Result<(), String> {
        let src = ctx.mem_block[self.src as usize];
        let dest = &mut ctx.mem_block[self.dest as usize];

        match self.alu_op {
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
            _ => return Err(format!("Invalid ALU operation: {}", self.alu_op)),
        }
        Ok(())
    }
}

pub struct Jump {
    op_code: u8,
    offset: i16,
}

impl Jump {
    pub fn new(op_code: u8, offset: i16) -> Self {
        Self { op_code, offset }
    }
}

impl Instruction for Jump {
    fn execute(&self, ctx: &mut Context) -> Result<(), String> {
        if self.offset < 0 && ctx.line_counter < (self.offset.abs() as usize) {
            return Err(String::from("invalid negative offset inside program"));
        }
        if self.offset > 0 && (ctx.program_size - ctx.line_counter) < (self.offset as usize) {
            return Err(String::from("invalid positive offset inside program"));
        }

        let new_line = ((ctx.line_counter as isize) + (self.offset as isize)) as usize;
        let zero_flag_set = (ctx.status_register & 1) == 0x01;
        let undeflow_flag_set = (ctx.status_register & (1 << 1)) == 0x02;
        match self.op_code {
            0x20 => ctx.line_counter = new_line,
            0x21 => if undeflow_flag_set { ctx.line_counter = new_line },
            0x22 => if undeflow_flag_set || zero_flag_set { ctx.line_counter = new_line },
            0x23 => if zero_flag_set { ctx.line_counter = new_line },
            _ => return Err(format!("Invalid jump operation: {}", self.op_code)),
        }
        Ok(())
    }
}

pub struct Process {
    context: Context,
    source: Vec<String>,
}

impl Process {
    pub fn new(source_code: String) -> Self {
        let source: Vec<String> = source_code
            .split('\n')
            .map(str::trim)
            .map(str::to_string)
            .collect();
        Self { context: Context::new(source.len()), source }
    }

    pub fn run(&mut self) {
        use super::parser;

        while self.context.line_counter < self.context.program_size {
            self.context.line_counter += 1;
            parser::instr()
                .parse(self.source[self.context.line_counter - 1].as_bytes())
                .unwrap()
                .execute(&mut self.context)
                .unwrap();
        }
    }
}
