// resources: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
mod cpu;

extern crate direct_gui;
extern crate minifb;

use std::fs::File;
use std::io::Read;


use direct_gui::*;
use minifb::*;

fn main() {
    let mut cpu = cpu::CPU::new();
    let mut file = File::open("games/INVADERS").unwrap(); //open a game.
    let mut program = Vec::<u8>::new();
    let _bytecount = file.read_to_end(&mut program); //read to a buffer
    cpu.load(&mut program); //load the program in CPU.
    cpu.load_sprites(); // initialise a new cpu
    let mut window = Window::new("Chip 8 Emulator", 640, 320, WindowOptions::default())
        .expect("Unable to open window");
    let screen_size = (640i32, 320i32);
    let mut buffer: Vec<u32> = vec![0; (screen_size.0 * screen_size.1) as usize];
    let mut gui = Gui::new(screen_size);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        cpu.cycle();
        //scale it up
        let mut scaled = [[false; 640]; 320];
        let mut x = 0;
        let mut y = 0;
        for column in scaled.iter_mut() {
            for row in column.iter_mut() {
                *row = cpu.display[x / 10][y / 10];
                y = y + 1;
            }
            y = 0;
            x = x + 1;
        }

        let mut k = 0;
        //flatten the scaled array and put that in the buffer.
        for (_i, &column) in scaled.iter().enumerate() {
            for (_j, &row) in column.iter().enumerate() {
                if row {
                    buffer[k] = 0xa4f644;
                } else {
                    buffer[k] = 0;
                }
                k = k + 1;
            }
        }
        gui.draw_to_buffer(&mut buffer);
        window.update_with_buffer_size(&buffer, 64, 32).unwrap();
    }
    cpu.cycle();
}

/* putting everything together */

/*will implement a runner*/