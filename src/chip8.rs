use crate::cpu::Cpu;
use crate::memory::Memory;

pub struct Chip8 {
    cpu: Cpu,
    ram: Memory,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            ram: Memory::new(),
        }
    }

    pub fn load(&mut self, buffer: Vec<u8>) {
        self.ram.load_rom(buffer);
    }

    pub fn dump(&self) {
        self.ram.debug_memory();
        self.cpu.debug_registers();
    }
}
