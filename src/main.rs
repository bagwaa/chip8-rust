use std::fs::File;
use std::io::prelude::Read;

use chip8::Chip8;
use clap::{App, Arg};

mod chip8;
mod memory;

fn main() {
    let args = App::new("chip8-rust")
        .version("0.1.0")
        .about("Rust implementation of the CHIP-8 interpreter")
        .arg(
            Arg::with_name("romfile")
                .help("The ROM file/path to load")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let romfile = args.value_of("romfile").expect("Unable to open ROM");

    let mut chip8 = Chip8::new();

    let mut file = File::open(romfile).expect("File not found");
    let mut buffer = Vec::<u8>::new();

    file.read_to_end(&mut buffer)
        .expect("Unable to read rom file");

    chip8.load(buffer);
    chip8.dump(); // debug the memory to see the sprites loaded and the game loaded
}
