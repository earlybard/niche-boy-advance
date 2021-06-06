use crate::emu::Emu;

#[derive(Debug)]
pub struct Memory {
    pub buffer: [u8; 0xFFFF + 1]
}

impl Emu {
    pub fn read_byte_from_memory(&mut self, addr: u16) -> u8 {
        self.cpu.cycle();
        self.memory.buffer[addr as usize]
    }

    pub fn write_byte_to_memory(&mut self, addr: u16, byte: u8) {
        self.cpu.cycle();
        self.memory.buffer[addr as usize] = byte;
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self { buffer: [0; 0xFFFF + 1] }
    }
}
