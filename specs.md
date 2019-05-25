# BelieVMr Specs

#### Instruction Set Assembly

| Instructions      | OpCode | Operations                                |
| ----------------- | ------ | ----------------------------------------- |
| mov literal, dest | 0x10   | dest = binary(literal)                    |
| add src, dest     | 0x11   | dest = dest + src                         |
| sub src, dest     | 0x12   | dest = dest - src                         |
| and src, dest     | 0x13   | dest = dest & src                         |
| or src, dest      | 0x14   | dest = dest \| src                        |
| xor src, dest     | 0x15   | dest = dest ^ src                         |
| cmp src, dest     | 0x16   | compare dest with src and set status bits |
| sll src, dest     | 0x17   | dest = dest << src                        |
| srl src, dest     | 0x18   | dest = dest >> src                        |
| sra src, dest     | 0x19   | dest = extended(dest) >> src              |
| j offset          | 0x20   | add offset to program counter             |
| jl offset         | 0x21   | jump if strictly less than bit set        |
| jle offset        | 0x22   | jump if less than or equal bits set       |
| je offset         | 0x23   | jump if only equal bits set               |

src and dest are 16 bit hexadecimal memory addresses prefixed with "0x"
offset is 16 bit decimal integer
literal is 32 bit decimal or hexadecimal prefixed with "0x" integer