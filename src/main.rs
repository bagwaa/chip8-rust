use std::fs::File;
use std::io::prelude::Read;

use chip8::Chip8;

mod chip8;
mod memory;

fn main() {
    let mut chip8 = Chip8::new();

    // should probably accept arguments to allow us to load a rom via path

    let mut file = File::open("roms/PONG").expect("File not found");
    let mut buffer = Vec::<u8>::new();

    file.read_to_end(&mut buffer)
        .expect("Unable to read rom file");

    chip8.load(buffer);
    chip8.dump();
}
