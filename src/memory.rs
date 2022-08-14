// 0x000 (0) to 0x1FF (511) reserved for interpreter (display) etc
// 0x200 (512) to 0xFFF (4095) PROGRAM MEMORY LOADED IN

const INTERPRETER_START_ADDRESS: usize = 0x000;
const MAX_ADDRESSABLE_MEMORY: usize = 4096;
const ROM_START_ADDRESS: usize = 0x200;

pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory {
            memory: vec![0; MAX_ADDRESSABLE_MEMORY],
        };

        let sprites = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];

        let mut sprite_index = INTERPRETER_START_ADDRESS;

        for sprite in sprites {
            for byte in sprite {
                memory.memory[sprite_index] = byte;
                sprite_index += 1;
            }
        }

        memory
    }

    pub fn load_rom(&mut self, buffer: Vec<u8>) {
        for (i, &byte) in buffer.iter().enumerate() {
            self.memory[ROM_START_ADDRESS + i] = byte;
        }
    }

    pub fn debug_memory(&self) {
        for (i, &byte) in self.memory.iter().enumerate() {
            println!("{:#x} ({}): {:#x} ({:#?})", i, i, byte, byte);
        }
    }
}
