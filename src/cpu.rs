/* Chip 8 Specs */
/* 
Memory: 4kb ;0x000 (0) to 0xFFF (4095). 
Registers: V0, V2, ...Vf (16 8 bit registers)
            sound and time registers; u8
            I: u16 -> Stores memory address
*/

extern crate rand;
use std::process;

pub struct CPU {
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
            memory: [0; 4096],
            stack: [0; 16],
            keystrokes: [false; 16], //maybe convert this to bool.
            display: [[true; 64]; 32],
        }
    }
    pub fn load(&mut self, program: &mut Vec<u8>){
        let mut data = vec![0; 0x200]; //program starts at address 0x200 so filling till there with 0
        data.append(program);
        for (index, &byte) in data.iter().enumerate(){
            self.memory[index] = byte; // load program into memory
        }
    }
    pub fn load_sprites(&mut self){
        // note to my stupid self: these are characters 
        let sprites: [u8; 80] = [0xF0, 0x90, 0x90, 0x90, 0xF0,  // 0
                                 0x20, 0x60, 0x20, 0x20, 0x70,  // 1
                                 0xF0, 0x10, 0xF0, 0x80, 0xF0,  // 2
                                 0xF0, 0x10, 0xF0, 0x10, 0xF0,  // 3
                                 0x90, 0x90, 0xF0, 0x10, 0x10,  // 4
                                 0xF0, 0x80, 0xF0, 0x10, 0xF0,  // 5
                                 0xF0, 0x80, 0xF0, 0x90, 0xF0,  // 6
                                 0xF0, 0x10, 0x20, 0x40, 0x40,  // 7
                                 0xF0, 0x90, 0xF0, 0x90, 0xF0,  // 8
                                 0xF0, 0x90, 0xF0, 0x10, 0xF0,  // 9
                                 0xF0, 0x90, 0xF0, 0x90, 0x90,  // A
                                 0xE0, 0x90, 0xE0, 0x90, 0xE0,  // B
                                 0xF0, 0x80, 0x80, 0x80, 0xF0,  // C
                                 0xE0, 0x90, 0x90, 0x90, 0xE0,  // D
                                 0xF0, 0x80, 0xF0, 0x80, 0xF0,  // E
                                 0xF0, 0x80, 0xF0, 0x80, 0x80];// F

        //filling memory till 0x200                       
        let mut data = vec![0; 0x200];
        for i in 0..80 {
            data[i] = sprites[i];
        }
        for (index, &byte) in data.iter().enumerate(){
            self.memory[index] = byte; // load items from data into self.memory
        }

    }
    pub fn cycle(&mut self) {
        self.get_opcode();
        self.execute();
    }
    fn inc_pc(&mut self) {
        self.pc += 2;
    }
    fn unimplemented(&self) {
        println!("Failed, ...Exiting");
        process::exit(0);
    }
    fn get_opcode(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
    }
    fn get_nnn(&self)-> u16 {  // returns nnn bit of the opcode. 1(nnn)
        self.opcode & 0x0fff
    }
    fn get_kk(&self) -> u8 { // returns kk bit of opcode 3x(kk)
        (self.opcode & 0x00ff) as u8
    }
    fn get_x(&self) -> u8 {  // returns x bit of opcode 3(x)kk
        ((self.opcode & 0x0f00) >> 8) as u8
    }
    fn get_y(&self) -> u8 {  //returns y bit of opcode 3x(y)z
        ((self.opcode & 0x00f0) >> 4) as u8 
    }

    fn execute(&mut self) {
        match self.opcode & 0xf000 {
            0x0000 => self.code_0xxx(), // 00e0 = clear screen; 00ee = return from subroutine; 0000 = exit
            0x1000 => self.op_jp(), // jump to nnn
            0x2000 => self.op_call(), // call a subroutine on nnn
            0x3000 => self.op_se(), // 3xkk; if v[x] == kk skip next instruction
            0x4000 => self.op_sne(), // 4xkk; if v[x] != kk skip next instruction
            0x5000 => self.op_se_xy(), //5xy0 vx == vy increment pc()
            0x6000 => self.op_ld_vx_byte(), //6xkk set Vx = kk
            0x7000 => self.op_add_vx_byte(), //yes i have gotten lazy
            0x8000 => self.code_8xxx(), //further matches with all possible 8xxx code
            0x9000 => self.op_sne_vx_vy(), //skip if vx not equal vy
            0xA000 => self.op_ld_i_addr(), 
            0xB000 => self.op_jp_v0_addr(),
            0xC000 => self.op_rnd_vx_byte(),
            0xD000 => self.op_drw_vx_vy_n(), // this was a pain.
            0xE000 => self.code_exxx(),
            0xF000 => self.code_fxxx(),
            _      => self.unimplemented()
        }
    }
    // instruction matching 0xxx format. 
    fn code_0xxx(&mut self) {
        match self.opcode {
            0x00E0 => self.op_cls(),
            0x00EE => self.op_ret(),
            0x0000 => { // this is the exit code.
                println!("Terminating");
                process::exit(0);
            }
            _    => self.unimplemented() //incase does not match the three known instruction set go back to exiting
        }
    }
    fn code_8xxx(&mut self) {
        match self.opcode & 0x000f {
            0   => self.op_ld_vx_vy(),
            1   => self.op_or(),
            2   => self.op_and(),
            3   => self.op_xor(),
            4   => self.op_add_vx_vy(),
            5   => self.op_sub_vx_vy(),
            6   => self.op_shr_vx_vy(),
            7   => self.op_subn_vx_vy(),
            0xE => self.op_shl_vx_vy(),
            _   => self.unimplemented()
        }
    }
    fn code_exxx(&mut self) {
        match self.opcode & 0x00FF {
            0x9E => self.op_skp_vx(),
            0xA1 => self.op_sknp_vx(),
            _    => self.unimplemented()
        }
    }
    fn code_fxxx(&mut self) {
        match self.opcode & 0x00FF {
            0x07 => self.op_ld_vx_dt(),
            0x0A => self.op_ld_vx_k(),
            0x15 => self.op_ld_dt_vx(),
            0x18 => self.op_ld_st_vx(),
            0x1E => self.op_add_i_vx(),
            0x29 => self.op_ld_f_vx(),
            0x33 => self.op_ld_b_vx(),
            0x55 => self.op_ld_i_vx(),
            0x65 => self.op_ld_vx_i(),
            _    => self.unimplemented()
        }
    }
    //clear screen operation
    fn op_cls(&mut self) {
        self.display = [[false; 64]; 32];
        self.inc_pc();
    }
    // Return from a subroutine.
    fn op_ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp] as usize;
        self.inc_pc();
    }
     //1nnn jump to location nnn
    fn op_jp(&mut self) {
        self.pc = self.get_nnn() as usize; 
    }
    //2nnn call subroutine at nnn
    fn op_call(&mut self) {
        self.stack[self.sp] = self.pc as u16;
        self.sp += 1;
        self.pc = self.get_nnn() as usize;
    }
    //3xkk skip if vx ==kk
    fn op_se(&mut self) {
        if self.v[self.get_x() as usize] == self.get_kk() {
            self.inc_pc();
        }
        self.inc_pc();
    }
    //4xkk skip if vx != kk
    fn op_sne(&mut self) {
        if self.v[self.get_x() as usize] != self.get_kk() {
            self.inc_pc();
        }
        self.inc_pc();
    }
    //5xy0 vx == vy increment pc()
    fn op_se_xy(&mut self) {
        if self.v[self.get_x() as usize] == self.v[self.get_y() as usize] {
            self.inc_pc()
        }
        self.inc_pc()
    }
    //6xkk set Vx = kk
    fn op_ld_vx_byte(&mut self) {
        self.v[self.get_x() as usize] = self.get_kk();
        self.inc_pc();
    }

    fn op_add_vx_byte(&mut self) {
        let x = self.get_x() as usize;
        self.v[x] = ((self.v[x] as u16) + (self.get_kk() as u16)) as u8;
        self.inc_pc();
    }
    fn op_ld_vx_vy(&mut self) {
        let x = self.get_x() as usize;
        let y = self.get_y() as usize;
        self.v[x] = self.v[y];
        self.inc_pc();
    }
    fn op_or(&mut self) {
        let x = self.get_x() as usize;
        let y = self.get_y() as usize;
        self.v[x] = self.v[x] | self.v[y];
        self.inc_pc();
    }
    fn op_and(&mut self) {
        let x = self.get_x() as usize;
        let y = self.get_y() as usize;
        self.v[x] = self.v[x] & self.v[y];
        self.inc_pc();
    }
    fn op_xor(&mut self) {
        let x = self.get_x() as usize;
        let y = self.get_y() as usize;
        self.v[x] = self.v[x] ^ self.v[y];
        self.inc_pc();
    }
    fn op_add_vx_vy(&mut self) {
        let x = self.get_x() as usize;
        let y = self.get_y() as usize;
        let sum = (self.v[x] as u16) + (self.v[y] as u16);

        if sum > 0xFF {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }

        self.v[x] = sum as u8;
        self.inc_pc();
    }
    fn op_sub_vx_vy(&mut self) {
        let x = self.get_x() as usize;
        let y = self.get_y() as usize;

        if self.v[x] > self.v[y] { 
            self.v[0xf] = 1;
        } else {
            self.v[0xf] = 0;
        }

        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        self.inc_pc();
    }
    fn op_shr_vx_vy(&mut self) {
        let x = self.get_x() as usize;
        let y = self.get_y() as usize;

        self.v[0xf] = self.v[y] & 1;
        self.v[x] = self.v[y] >> 1;
        self.inc_pc();
    }
    fn op_subn_vx_vy(&mut self) {
        let x = self.get_x() as usize;
        let y = self.get_y() as usize;

        if self.v[y] > self.v[x] { 
            self.v[0xf] = 1;
        } else {
            self.v[0xf] = 0;
        }

        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        self.inc_pc();
    }
    // 8xyE - SHL Vx, Vy -- Set Vx = Vy SHL 1
    fn op_shl_vx_vy(&mut self) {
        let x = self.get_x() as usize;
        let y = self.get_y() as usize;
        self.v[0xf] = self.v[y]>> 7;
        self.v[x] = self.v[y] << 1;
        self.inc_pc();
    }
    fn op_sne_vx_vy(&mut self) {
        if self.v[self.get_x() as usize] != self.v[self.get_y() as usize] {
            self.inc_pc();
        }
        self.inc_pc();
    }
    fn op_ld_i_addr(&mut self) {
        self.i = self.get_nnn();
        self.inc_pc();
    }
    fn op_jp_v0_addr(&mut self) {
        self.pc = self.v[0] as usize + self.get_nnn() as usize;
    }

    fn op_rnd_vx_byte(&mut self) {
        self.v[self.get_x() as usize] = rand::random::<u8>() & self.get_kk();
        self.inc_pc();
    }
    fn op_drw_vx_vy_n(&mut self) {
      // will draw here 
      /*
        The interpreter reads n bytes from memory, starting at the address stored in I. 
        These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). 
        Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, 
        otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, 
        it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4
    */
        let vx = self.v[self.get_x() as usize] as usize; // get x 
        let vy = self.v[self.get_y() as usize] as usize; // get y 
        let n = (self.opcode & 0x000f) as u8  as usize; // get z 
        let i = self.i as usize; // I 
        let mut flipped = false; 

        if (vx > 0x3F) | (vy > 0x1F) { 
            return; 
        }
        {
            let sprite = &self.memory[i .. i + n];
            for row_count in 0 .. n {
                if vy + row_count > 31 {  //wrapping around the screen.
                    break; 
                }
                let row = &mut self.display[row_count + vy];
                let vxp8;
                if vx + 8 > 64 { 
                    vxp8 = 64; 
                } else { 
                    vxp8 = vx + 8; 
                }
                let slice = &mut row[vx .. vxp8];
                let current_sprite = &sprite[row_count];
                for pixel in 0..slice.len() {
                        let sprite_pixel = current_sprite & (0x80 >> pixel) != 0;
                        if slice[pixel] & sprite_pixel {
                            flipped = true;
                        }
                        slice[pixel] = slice[pixel] ^ sprite_pixel; //if both are true you get 0, if both are false you get 1
                }

            }
        }

        if flipped { self.v[0xF] = 1} else { self.v[0xF] = 0 }
        self.inc_pc();
    }
    fn op_skp_vx(&mut self) {
        let key = self.v[self.get_x() as usize] as usize;
        if self.keystrokes[key] {
            self.inc_pc();
        }
        self.inc_pc();
    }
    fn op_sknp_vx(&mut self) {
        let key = self.v[self.get_x() as usize] as usize;
        if !self.keystrokes[key] {
            self.inc_pc();
        }
        self.inc_pc();
    }

    fn op_ld_vx_dt(&mut self) {
        self.v[self.get_x() as usize] = self.delay;
        self.inc_pc();
    }
    fn op_ld_vx_k(&mut self) {
        let mut continue_exec = false;
        for (key, pressed) in self.keystrokes.iter().enumerate() {
            if *pressed {
                self.v[self.get_x() as usize] = key as u8;
                continue_exec = true;
            }
        }
        if continue_exec {
            self.inc_pc();
        }
    }
    fn op_ld_dt_vx(&mut self) {
        self.delay = self.v[self.get_x() as usize];
        self.inc_pc();
    }
    // DT is set equal to the value of Vx.
    fn op_ld_st_vx(&mut self) {
        self.sound = self.v[self.get_x() as usize];
        self.inc_pc();
    }
    fn op_add_i_vx(&mut self) {
        self.i = self.i + self.v[self.get_x() as usize] as u16;
        self.inc_pc();
    }
    fn op_ld_f_vx(&mut self) {
        self.i = (self.v[self.get_x() as usize] * 5) as u16;
        self.inc_pc();
    }
    fn op_ld_b_vx(&mut self) {
        let vx = self.v[self.get_x() as usize];
        let i = self.i as usize;
        self.memory[i] = vx / 100;
        self.memory[i + 1] = (vx / 10) % 10;
        self.memory[i + 2] = (vx %100) %10;
        self.inc_pc();
    }
    fn op_ld_i_vx(&mut self) {
        let x = self.get_x() as u16;
        let i = self.i;
        for n in 0..=x {
            self.memory[(i + n) as usize] = self.v[n as usize];
        }
        self.i = i + x + 1;
        self.inc_pc();
    }
    fn op_ld_vx_i(&mut self) {
        let x = self.get_x() as u16;
        let i = self.i;
        for n in 0..=x {
            self.v[n as usize] = self.memory[(i + n) as usize];
        }
        self.i = i + x + 1;
        self.inc_pc();
    }
    

}