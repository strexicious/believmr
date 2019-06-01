# BelieVMr Specs

#### Instruction Set Assembly

| Instructions      | OpCode | Operations                          |
| ----------------- | ------ | ----------------------------------- |
| mov integer, dest | 0x10   | dest = binary(integer)              |
| add src, dest     | 0x11   | dest = dest + src                   |
| sub src, dest     | 0x12   | dest = dest - src                   |
| and src, dest     | 0x13   | dest = dest & src                   |
| or src, dest      | 0x14   | dest = dest \| src                  |
| xor src, dest     | 0x15   | dest = dest ^ src                   |
| cmp src, dest     | 0x16   | dest - src, setting the status bits |
| sll src, dest     | 0x17   | dest = dest << src                  |
| srl src, dest     | 0x18   | dest = dest >> src                  |
| sra src, dest     | 0x19   | dest = extended(dest) >> src        |
| j offset          | 0x20   | add offset to program counter       |
| jl offset         | 0x21   | jump if status[1] set               |
| jle offset        | 0x22   | jump if status[0] or status[1] set  |
| je offset         | 0x23   | jump if status[0] set               |



#### Grammar

`src` and `dest` are memory addresses. Memory addresses are 16 bits hexadecimal values. Hexadecimal values must be prefixed with "0x". `offset` is a 16 bit decimal value. `integer` is 32 bit decimal value with sign or hexadecimal representation of it in two's complement.

``` 
instr	 ::= movInstr | aluInstr | jmpInstr
movInstr ::= "mov" integer "," HEX_NUMBER
aluInstr ::= aluOp HEX_NUMBER "," HEX_NUMBER
jmpInstr ::= jCond offset
integer  ::= DEC_NUMBER | HEX_NUMBER
aluOp	 ::= "add" | "sub" | "and" | "or" | "xor" | "cmp" | "sll" | "srl" | "sra"
jCond	 ::= "j" | "jl" | "jle" | "je"
```



#### Status Register

An 8 bit register where the bits as used as described:

- bit 0 - Zero flag,
- bit 1 - Underflow flag,
- bit 2 - Overflow flag,
- bits 3..7 - Reserved



#### About jumps

The program counter is incremented before decodification, thus, jumps must take into account this information. `offset` can at max cause the program counter to reach program's size causing it to halt.