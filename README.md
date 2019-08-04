# BelieVMr Specs

[![issues](https://img.shields.io/github/issues/strexicious/believmr?style=flat-square)](https://github.com/strexicious/believmr/issues)
[![repo size](https://img.shields.io/github/repo-size/strexicious/believmr?style=flat-square)](https://github.com/strexicious/believmr)
[![code size](https://img.shields.io/github/languages/code-size/strexicious/believmr?style=flat-square)](https://github.com/strexicious/believmr)
[![license](https://img.shields.io/badge/license-Unlicense-blue.svg?style=flat-square)](./LICENSE)

## Instruction Set Assembly

| Instructions     | OpCode | Operations                                |
| ---------------- | ------ | ----------------------------------------- |
| print src        | 0x00   | prints a null terminated string           |
| mov integer dest | 0x10   | dest = binary(integer)                    |
| movm src dest    | 0x11   | dest = src                                |
| add src dest     | 0x12   | dest = dest + src                         |
| sub src dest     | 0x13   | dest = dest - src                         |
| and src dest     | 0x14   | dest = dest & src                         |
| or src dest      | 0x15   | dest = dest \| src                        |
| xor src dest     | 0x16   | dest = dest ^ src                         |
| cmp src dest     | 0x17   | dest - src, setting the status bits       |
| sll src dest     | 0x18   | dest = dest << src                        |
| srl src dest     | 0x19   | dest = dest >> src                        |
| sra src dest     | 0x1A   | dest = extended(dest) >> src              |
| mul src dest     | 0x1B   | dest = dest * src, xr0 = high(dest * src) |
| div src dest     | 0x1C   | dest = dest / src, xr0 = dest % src       |
| mde dest         | 0x1D   | dest = % or high of last mul/div          |
| j offset         | 0x20   | add offset to program counter             |
| jl offset        | 0x21   | jump if status[1] set                     |
| jle offset       | 0x22   | jump if status[0] or status[1] set        |
| je offset        | 0x23   | jump if status[0] set                     |
| subrout size     | 0x30   | subroutine with size # of params          |
| call line src    | 0x31   | call subroutine at index                  |
| return dest      | 0x32   | returns to instr before subroutine        |

## Endianess

By default we assume big endian format for our virtual machine.

## Status Register

An 8 bit register where the bits as used as described:

- bit 0 - Zero flag,
- bit 1 - Underflow flag,
- bit 2 - Overflow flag,
- bit 3 - System call invoked,
- bits 4..7 - Reserved

## Special Registers

- xr0 (32 bits) - Stores upper part of the multiplication or the remainder of a division

## Subroutine Table

Any instruction can be marked as the start of a new subroutine with a leading subrout instruction, and  `size` gives the number of parameters that it accepts. This instruction's line number is recorded along with the `size` in the subroutine table and is given the index [tables length - 1]. A subroutine can be invoked with `call` given an index into the table and `src` from where to fetch the arguments to the subroutine.

## About jumps

The program counter is incremented before decodification, thus, jumps must take into account this information. `offset` can at max cause the program counter to reach program's size causing it to halt.

