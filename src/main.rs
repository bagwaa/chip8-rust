mod chip8;

use chip8::cpu::Cpu;

fn main() {
    // Create a new CPU
    let mut cpu = Cpu::new();

    // Load ROM into memory from file specified as command line argument
    let rom_path = std::env::args().nth(1).expect("ROM file not found");

    // Load ROM file into RAM starting at 0x200
    cpu.load_rom(&rom_path);

    // Run the Emulator loop
    cpu.run();
}
