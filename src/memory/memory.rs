#[derive(Debug)]
pub struct Memory {
    pub buffer: [u8; 0xFFFF + 1]
}

impl Memory {
    pub fn read_byte(&self, index: u16) -> u8 {
        self.buffer[index as usize]
    }

    pub fn write_byte(&mut self, index: u16, byte: u8) {
        self.buffer[index as usize] = byte;
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self { buffer: [0; 0xFFFF + 1] }
    }
}
