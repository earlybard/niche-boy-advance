use crate::emu::Emu;
use crate::util::Util;

#[derive(Debug)]
pub struct Memory {
    pub buffer: [u8; 0xFFFF + 1]
}

impl Emu {
    pub fn read_byte_from_memory(&mut self, addr: u16) -> u8 {
        self.cycle();

        // if (0xFF00..=0xFF07).contains(&addr)
        //     || addr == 0xFF40
        //     || (0x8000..=0x97FF).contains(&addr) {
        //     eprintln!("read addr = {:#04X?}", addr);
        // }

        self.memory.buffer[addr as usize]
    }

    pub fn write_byte_to_memory(&mut self, addr: u16, byte: u8) {
        self.cycle();

        // if (0xFF00..=0xFF07).contains(&addr)
        //     || addr == 0xFF40
        //     || (0x8000..=0x97FF).contains(&addr) {
        //     eprintln!("write addr = {:#04X?}", addr);
        // }

        self.memory.buffer[addr as usize] = byte;
    }

    pub fn write_word_to_memory(&mut self, addr: u16, word: u16) {
        let (msb, lsb) = Util::word_to_bytes(word);
        self.write_byte_to_memory(addr, lsb);
        self.write_byte_to_memory(addr.wrapping_add(1), msb);
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self { buffer: [0; 0xFFFF + 1] }
    }
}
