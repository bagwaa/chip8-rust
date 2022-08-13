use std::fs::File;
use std::io::prelude::*;

use memory::Memory;

mod memory;

fn main() {
    // should probably accept arguments to allow us to load a rom via path

    // load the game from file and into a mutable empty buffer
    let mut file = File::open("roms/PONG").unwrap(); // handle this properly, and why mutable?
    let mut buffer = Vec::<u8>::new();

    file.read_to_end(&mut buffer).unwrap(); // handle this properly

    // Load the ROM in the buffer into a structure representing the CHIP-8 memory spec
    let mut memory = Memory::new();
    memory.load_rom(buffer);

    // quick dirty dump of the memory layout
    memory.debug_memory();
}
