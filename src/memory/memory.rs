use crate::emu::Emu;
use crate::gpu::gpu::GpuMode;
use crate::memory::memory_writers::{dma};
use crate::util::Util;
use log::Level::Trace;

#[derive(Debug)]
pub struct Memory {
    pub buffer: [u8; 0xFFFF + 1],
    pub bytes_read_this_instruction: [u8; 5],
    pub bytes_read_count: usize
}

impl Emu {
    pub fn read_byte_from_memory(&mut self, addr: u16) -> u8 {
        self.cycle();

        // TODO way less switching. We need views into this memory?

        if log_enabled!(Trace) {
            self.memory.bytes_read_this_instruction[self.memory.bytes_read_count] =
                self.memory.buffer[addr as usize];

            self.memory.bytes_read_count += 1;
        }

        let response = match addr {
            0x8000..=0x9FFF => {
                if self.gpu.get_mode() == GpuMode::PixelTransfer {
                    0xFF
                } else {
                    self.memory.buffer[addr as usize]
                }
            }
            0xFF40 => {
                // println!("lcd control READ {:?}", self.gpu.lcd_control);
                self.gpu.lcd_control.get_byte()
            },
            0xFF41 => {
                // println!("lcd status READ {:?}", self.gpu.lcd_status);
                self.gpu.lcd_status.get_byte()
            },
            0xFF44 => self.gpu.ly,
            0xFF0F => self.interrupts.interrupt_flag.get_byte(),
            0xFFFF => self.interrupts.interrupt_enable.get_byte(),
            _ => self.memory.buffer[addr as usize]
        };

        response

            // if (0xFF00..=0xFF07).contains(&addr)
        //     || addr == 0xFF40
        //     || (0x8000..=0x97FF).contains(&addr) {
        //     eprintln!("read addr = {:#04X?}", addr);
        // }
    }

    fn _write(&mut self, addr: u16, byte: u8) {
        self.memory.buffer[addr as usize] = byte;
    }

    pub fn write_byte_to_memory(&mut self, addr: u16, byte: u8) {
        self.cycle();

        match addr {
            0x8000..=0x9FFF => {
                // During pixel transfer, can't write to VRAM
                if self.gpu.get_mode() != GpuMode::PixelTransfer { self._write(addr, byte); }
            }
            0xFF40 => {
                self.gpu.lcd_control.set_byte(byte);
                println!("lcd control {:?}", self.gpu.lcd_control);
            },
            0xFF41 => {
                self.gpu.lcd_status.set_byte(byte);
                // and set mode
                println!("lcd status {:?}", self.gpu.lcd_status);
            },
            /// Any writes to LY while the LCD is enabled are ignored.
            /// When the LCD is disabled, LY is forcibly set to 0,
            /// and since it's read-only, the value never changes
            0xFF44 => { /* Writing to LY is illegal */ } //
            0xFF45 => {
                self.lyc_check();
                self._write(addr, byte);
            }
            0xFF46 => dma(&mut self.memory.buffer, byte),
            0xFF0F => self.interrupts.interrupt_flag.set_byte(byte),
            0xFFFF => self.interrupts.interrupt_enable.set_byte(byte),
            _ => self._write(addr, byte)
        }


        // if (0xFF00..=0xFF07).contains(&addr)
        //     || addr == 0xFF40
        //     || (0x8000..=0x97FF).contains(&addr) {
        //     eprintln!("write addr = {:#04X?}", addr);
        // }

    }

    pub fn write_word_to_memory(&mut self, addr: u16, word: u16) {
        let (msb, lsb) = Util::word_to_bytes(word);
        self.write_byte_to_memory(addr, lsb);
        self.write_byte_to_memory(addr.wrapping_add(1), msb);
    }

}

impl Default for Memory {
    fn default() -> Self {
        Self {
            buffer: [0; 0xFFFF + 1],
            bytes_read_count: 0,
            bytes_read_this_instruction: [0; 5]
        }
    }
}
