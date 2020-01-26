# CHIP 8 emulator

This is rough implementation of chip 8 emulator. 

http://devernay.free.fr/hacks/chip8/C8TECH10.HTM for more details about CHIP8.

![](./images/display.png)



## Things still left to implement.
- Taking input from keys.
- Timer for sound and delay.
- resize and scale of display 
- runner for the program.
- maybe also make it a wasm program.


## CPU

This is our CPU, the process is kind of simple. 
Load a program into memory, read an opcode from it, perform the matching operation.


```rust 
CPU {
    opcode: u16,  //2byte long 
    v: [u8; 16], //16 8 bit registers, will be called as V[0], V[1] .... last register is reserved.
    i: u16, // 16 bit register
    sound: u8, 
    delay: u8, // sound and delay decrease if not 0 at rate of 60Hz
    pc: usize, // program counter -> stores the currently executing address
    sp: usize, //pointer to top of stack.
    memory: [u8; 4096], // 4kb memory :)
    stack: [u16; 16], // our friendly old stack
    pub keystrokes: [bool; 16], //so apparently chip 8 has 16 keys
    pub display: [[bool;64]; 32] //gonna admit this is much better than [][]
}
```

keystrokes, sound and delay don't do anything for now, except I needed to declare them to implement the opcodes.

## Credits

[Hammster](https://github.com/Hammster) for the suggestion of building this and his help during it. 
[direct-gui](https://github.com/tversteeg/direct-gui).
