use std::fs::File;
use std::io::prelude::Read;

use chip8::Chip8;

mod chip8;
mod memory;

fn main() {
    // should probably accept arguments to allow us to load a rom via path

    // load the game from file and into a mutable empty buffer
    let mut file = File::open("roms/PONG").unwrap(); // handle this properly, and why mutable?
    let mut buffer = Vec::<u8>::new();

    file.read_to_end(&mut buffer).unwrap(); // handle this properly

    let mut chip8 = Chip8::new();
    chip8.load(buffer);
    chip8.debug();
}
