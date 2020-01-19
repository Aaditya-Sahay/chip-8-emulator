/* Chip 8 Specs */
/* 
Memory: 4kb ;0x000 (0) to 0xFFF (4095). 
Registers: V0, V2, ...Vf (16 8 bit registers)
            sound and time registers; u8
            I: u16 -> Stores memory address
*/


pub struct CPU {
    opcode: u16,  //2byte long 
    v: [u8; 16], //16 8 bit registers, will be called as V[0], V[1] .... last register is reserved.
    i: u16, // 16 bit register
    sound: u8, 
    delay: u8, // sound and delay decrease if not 0 at rate of 60Hz
    pc: usize, // program counter -> stores the currently executing address
    sp: usize, //pointer to top of stack.
    memory: [u8; 4096] // 4kb memory :)
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            opcode: 0,
            v: [0; 16],
            i: 0x200,
            sound: 0,
            delay: 0, 
            pc: 0x200,
            sp: 0,
            memory: [0; 4096]
        }
    }
    pub fn load(&mut self, program: &mut Vec<u8>){
        let mut data = vec![0; 0x200]; //program starts at address 0x200 so filling till there with 0
        data.append(program);
        for (index, &byte) in data.iter().enumerate(){
            self.memory[index] = byte; // load program into memory :)
        }
    }
    fn get_opcode(&mut self) {
        self.pc += 1;
        self.opcode = self.memory[self.pc] as u16; // get the next opcode
    }



}