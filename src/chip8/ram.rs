const MEMORY_SIZE: usize = 4096; // 4096 bytes of memory (4k)

#[derive(Debug)]
pub struct Ram {
    pub data: [u8; MEMORY_SIZE],
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            data: [0; MEMORY_SIZE],
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.data[address] = value;
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.data[address]
    }
}
