// resources: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
mod cpu;

use std::fs::File;
use std::io::Read;


fn main() {
    let mut cpu = cpu::CPU::new(); // initialise a new cpu 
    let mut file = File::open("games/PONG").unwrap(); //open a game.
    let mut program = Vec::<u8>::new();
    let _bytecount = file.read_to_end(&mut program); //read to a buffer 
    cpu.load(&mut program); //load the program in CPU.
    println!("{:?}", program);
}
