use colored::Colorize;

pub struct Cpu {
    registers: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: vec![0; 16],
        }
    }

    pub fn debug_registers(&self) {
        println!(
            "{}",
            "------ CPU REGISTERS (16 x 8 bit registers) ------"
                .to_string()
                .on_green()
                .black()
        );
        for (i, &byte) in self.registers.iter().enumerate() {
            println!("{:#x} ({}): {:#x} ({:#?})", i, i, byte, byte);
        }
    }
}
