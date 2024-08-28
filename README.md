# Byte Machine

This repository contains an 8-bit virtual machine implementation, complete with an assembler and an instruction set.

The Byte Machine features a 16-bit address space, allowing it to address up to 65535 memory locations. Each memory location holds an 8-bit value (u8).

The opcode is 4bit long which gives us 16 number of opcodes to have in byte machine.
There are 3 layout of instructions 1 byte, 2 bytes and 3 byte instructions generally the 3 bytes instruction are the ones which store the memory address.

## Run
```bash
git clone https://github.com/ShashwatAgrawal20/byte_machine.git && cd byte_machine
```

> This program only depends on the `anyhow` will probably remove that as well in the future just don't need that crap.
```bash
cargo run --bin asm vm/code.naked > vm/code.bin && cargo run vm/code.bin
```

## Registers

The Byte Machine includes a set of 8 registers:
- A, B, C, D: General-purpose 8-bit registers.
- SP (Stack Pointer): Points to the top of the stack.
- PC (Program Counter): Holds the address of the next instruction to be executed.
- BP (Base Pointer): Used for base-relative addressing in stack operations.
- Flags: A special register for condition flags like Zero and Overflow.

## Instructions
```
Nop,                                        // 0000 0000
Push(u8),                                   // 0001 xxxx | iiiiiiii
PopRegister(Registers),                     // 0010 rrrr
PushRegister(Registers),                    // 0011 rrrr
AddStack,                                   // 0100 0000
LoadImmediate(Registers, u8),               // 0101 rrrr | iiiiiiii
LoadMemory(Registers, u16),                 // 0110 rrrr | aaaaaaaa | aaaaaaaa
Store(Registers, u16),                      // 0111 rrrr | aaaaaaaa | aaaaaaaa
ALU(ALUOperation, Registers, Registers),    // 1000 oooo | rrrr | rrrr
Jump(JumpTarget),                           // 1001 0000 | aaaaaaaa | aaaaaaaa
JumpConditional(JumpCondition, JumpTarget), // 1010 cccc | aaaaaaaa | aaaaaaaa
Interrupt(u8),                              // 1111 iiii
```


## Example program
```
LoadImmediate A 0        				; Load the value 0 into register A
LoadImmediate B 5        				; Load the value 5 into register B
Store A 0xfffc           				; Store the value in register A (which is 0) into memory address 0xfffc

loop:
    ALU Sub A B          				; Subtract the value in register B from the value in register A (A = A - B)
    JumpConditional LT incrementer 		; If the result of A - B is less than zero (A < B), jump to the 'incrementer' label
    Jump end             				; If the result of A - B is not less than zero, jump to the 'end' label
incrementer:
    LoadMemory A 0xfffc  				; Load the value from memory address 0xfffc into register A
    LoadImmediate C 1    				; Load the value 1 into register C
    ALU Add A C          				; Add the value in register C (which is 1) to the value in register A (A = A + 1)
    Store A 0xfffc       				; Store the updated value in register A back into memory address 0xfffc
    Jump loop            				; Jump back to the 'loop' label to repeat the process

end:
    LoadMemory A 0xfffc  				; Load the final value from memory address 0xfffc into register A
    Interrupt 15         				; Trigger interrupt 15, possibly to end the program or signal completion

```

The equivelent high level program will look something like

```rust
let mut a: u8 = 0;
while a < 5 {
    a += 1;
}
```


## Assembler

The assembler is a single file which just writes the byte equivelent of the instructions which the vm can understand and it just dumps that to the stdout,
so please note you have redirect that to seperate file.

The assembler takes care of the lables beautifully btw, it just goes through the code ones(pass one) and actually stores the lables and their respective memory address in the hashmap
and during the second pass whereever it come accross those jump statement it just replace those lables with the memory address stored in the hashmap.
