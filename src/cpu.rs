use colored::Colorize;
use std::fmt::Display;

pub struct Cpu {
    registers: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
    dt: u16,
    st: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0; 16],
            dt: 0,
            st: 0,
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

        println!(
            "{}",
            "------ CPU STACK REGISTER (16 x 16 bit registers) ------"
                .to_string()
                .on_green()
                .black()
        );

        for (i, &byte) in self.stack.iter().enumerate() {
            println!("{:#x} ({}): {:#x} ({:#?})", i, i, byte, byte);
        }

        self.single_register_dump("------ I REGISTER (1 x 16 bit register) ------", self.i);

        self.single_register_dump(
            "------ PC (Program Count) REGISTER (1 x 16 bit register) ------",
            self.pc,
        );

        self.single_register_dump(
            "------ SP (Stack Pointer) REGISTER (1 x 8 bit register) ------",
            self.sp,
        );

        self.single_register_dump(
            "------ DT (Delay Timer) REGISTER (1 x 16 bit register) ------",
            self.dt,
        );

        self.single_register_dump(
            "------ ST (Sound Timer) REGISTER (1 x 16 bit register) ------",
            self.st,
        );
    }

    fn single_register_dump<T: Display>(&self, message: &str, data: T) {
        println!("{}", message.to_string().on_green().black());
        println!("{}", data);
    }
}
