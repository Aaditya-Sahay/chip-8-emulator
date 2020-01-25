// resources: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
mod cpu;

extern crate direct_gui;
extern crate minifb;


use std::fs::File;
use std::io::Read;

//@https://github.com/Hammster's suggestion && https://github.com/tversteeg/direct-gui
use direct_gui::*;
use minifb::*;


fn main() {
    let mut cpu = cpu::CPU::new(); // initialise a new cpu 
    let mut file = File::open("games/PONG").unwrap(); //open a game.
    let mut program = Vec::<u8>::new();
    let _bytecount = file.read_to_end(&mut program); //read to a buffer 
    cpu.load(&mut program); //load the program in CPU.
    cpu.load_sprites(); //load sprites
    let mut window = Window::new("chip 8 emu - ESC to exit", 64, 32, WindowOptions::default()).expect("Unable to open window");
    let screen_size = (64i32, 32i32);
    let mut buffer: Vec<u32> = vec![0; (screen_size.0 * screen_size.1) as usize];
    let mut gui = Gui::new(screen_size);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, &column) in cpu.display.iter().enumerate(){
            for (j, &row) in column.iter().enumerate() {
                if row {
                   buffer[i*j] = 0xffffff;
                    
                }else {
                    buffer[i*j] = 0;
                }
            }
        }
        gui.draw_to_buffer(&mut buffer);
        window.update_with_buffer_size(&buffer, 64, 32).unwrap();
    }
    cpu.cycle();
}
