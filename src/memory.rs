// 0x000 (0) to 0x1FF (511) reserved for interpreter (display) etc
// 0x200 (512) to 0xFFF (4095) PROGRAM MEMORY LOADED IN

const ROM_START_ADDRESS: usize = 0x200;
const MAX_ADDRESSABLE_MEMORY: usize = 4096;

pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: vec![0; MAX_ADDRESSABLE_MEMORY],
        }
    }

    pub fn load_rom(&mut self, buffer: Vec<u8>) {
        for (i, &byte) in buffer.iter().enumerate() {
            self.memory[ROM_START_ADDRESS + i] = byte;
        }
    }

    pub fn debug_memory(&self) {
        for (i, &byte) in self.memory.iter().enumerate() {
            println!("{:#x}: {:#?}", i, byte);
        }
    }
}
