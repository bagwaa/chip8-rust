const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

use super::ram::Ram;

#[derive(Debug)]
pub struct Cpu {
    // 4k of memory, each byte is 8 bits
    ram: Ram,
    // 16-bit program counter
    pc: usize,
    // 16 8-bit general purpose registers
    v: [u8; 16],
    // 16-bit index register (used for memory addresses)
    i: u16,
    // 16 8-bit stack
    stack: [u16; 16],
    // 8-bit stack pointer
    sp: u8,
    // 8-bit delay timer
    dt: u8,
    // 8-bit sound timer
    st: u8,
    // 16 8-bit keypad
    keys: [bool; 16],
    // 64x32 monochrome display
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            ram: Ram::new(),
            pc: 0x200,
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            sp: 0,
            dt: 0,
            st: 0,
            keys: [false; 16],
            display: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        };

        cpu.ram.load_fontset();

        cpu
    }

    pub fn get_display(&self) -> &[bool] {
        &self.display
    }

    pub fn load_rom(&mut self, rom_path: &str) {
        // Read ROM file into memory starting at 0x200
        let rom = std::fs::read(rom_path).expect("Failed to read ROM file");

        for (i, &byte) in rom.iter().enumerate() {
            self.ram.write_byte(0x200 + i, byte);
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0x200;
        self.v = [0; 16];
        self.i = 0;
        self.stack = [0; 16];
        self.sp = 0;
        self.dt = 0;
        self.st = 0;
        self.keys = [false; 16];
        self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // BEEP
            }
            self.st -= 1;
        }
    }

    pub fn stack_push(&mut self, value: u16) {
        self.stack[self.sp as usize] = value;
        self.sp += 1;
    }

    pub fn stack_pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    fn fetch(&mut self) -> u16 {
        // Each opcode consists of two bytes, so we need to combine them into a single u16
        // The first byte is the high byte, the second byte is the low byte
        let high_byte = self.ram.read_byte(self.pc) as u16; // example: 0x12 (8-bit opcode)
        let low_byte = self.ram.read_byte(self.pc + 1) as u16; // example: 0x34 (8-bit opcode)

        // Combine the two bytes into a single opcode using bitwise OR (|) and bit shifting (<<)
        // For example, if the high byte is 0x12 and the low byte is 0x34, the opcode would be
        // 0x1234
        // example: 0x1234 (16-bit opcode)

        // Move program counter to next opcode
        self.pc += 2;

        // Return the opcode
        (high_byte << 8) | low_byte
    }

    fn decode(&self, opcode: u16) -> (u8, u8, u8, u8) {
        // Decode opcode
        let digit1 = (opcode & 0xF000) >> 12; // Get first digit of opcode
        let digit2 = (opcode & 0x0F00) >> 8; // Get second digit of opcode
        let digit3 = (opcode & 0x00F0) >> 4; // Get third digit of opcode
        let digit4 = opcode & 0x000F; // Get fourth digit of opcode

        (digit1 as u8, digit2 as u8, digit3 as u8, digit4 as u8)
    }

    fn execute(&mut self, digit1: u8, digit2: u8, digit3: u8, digit4: u8) {
        // Match opcode and execute instruction
        match (digit1, digit2, digit3, digit4) {
            // NOP
            (0, 0, 0, 0) => (),
            // CLS
            (0, 0, 0xE, 0) => {
                // Clear the display
                self.display = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            // Unknown Opcode Print an Error
            _ => println!("Unknown opcode"),
        }
    }

    pub fn tick(&mut self) {
        // fetch
        let opcode = self.fetch();
        // decode
        let (digit1, digit2, digit3, digit4) = self.decode(opcode);
        // execute
        self.execute(digit1, digit2, digit3, digit4);
    }
}
