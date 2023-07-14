use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;
use crate::emu::Emu;
use crate::gpu::gpu::GpuMode;
use crate::memory::memory_writers::{dma};
use crate::util::Util;
use log::Level::Trace;
use crate::memory::addresses::{LY, LYLC};
use crate::memory::bitsets::{LCDControl, LCDStatus};

#[derive(Debug)]
pub struct Memory {
    pub buffer: [u8; 0xFFFF + 1],
    pub bytes_read_this_instruction: [u8; 5],
    pub bytes_read_count: usize
}

type Buffer = [u8; 0xFFFF + 1];

/// Only use these with known safe values.
/// CPU instructions that can read or modify any memory must go via `read_byte_from_memory`
impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.buffer[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.buffer[index as usize]
    }
}

impl Emu {
    pub fn read_byte_from_memory(&mut self, addr: u16) -> u8 {
        self.cycle();

        if log_enabled!(Trace) {
            self.memory.bytes_read_this_instruction[self.memory.bytes_read_count] =
                self.memory[addr];

            self.memory.bytes_read_count += 1;
        }

        let response = match addr {
            0x8000..=0x9FFF => {
                if self.get_gpu_mode() == GpuMode::PixelTransfer {
                    0xFF
                } else {
                    self.memory[addr]
                }
            }
            _ => self.memory[addr]
        };

        response
    }

    pub fn write_byte_to_memory(&mut self, addr: u16, byte: u8) {
        self.cycle();

        match addr {
            0x8000..=0x9FFF => {
                // During pixel transfer, can't write to VRAM
                if self.get_gpu_mode() != GpuMode::PixelTransfer { self.memory[addr] = byte; }
            }
            // Any writes to LY while the LCD is enabled are ignored.
            // When the LCD is disabled, LY is forcibly set to 0,
            // and since it's read-only, the value never changes
            LY => { /* Writing to LY is illegal */ } //
            LYLC => {
                self.lyc_check();
                self.memory[addr] = byte;
            }
            0xFF46 => dma(&mut self.memory.buffer, byte),
            _ => self.memory[addr] = byte
        }
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
