use std::fmt::{Debug};
use crate::util::Util;
use crate::gpu::gpu::GPU;
use crate::cpu::cpu::CPU;
use crate::registers::registers::Registers;
use crate::memory::memory::Memory;
use crate::interrupts::interrupts::Interrupts;

#[derive(Debug)]
#[derive(Default)]
pub struct Emu {
    pub cpu: CPU,
    pub registers: Registers,
    pub memory: Memory,
    pub gpu: GPU,
    pub interrupts: Interrupts
}

#[allow(dead_code)]
impl Emu {

    pub fn read_and_inc(&mut self) -> u8 {
        self.registers.program_counter += 1;
        self.read_byte_from_memory(self.registers.program_counter - 1)
    }

    pub fn read_u16_and_inc(&mut self) -> u16 {
        let lsb = self.read_and_inc() as u16;
        let msb = self.read_and_inc() as u16;

        // msb = 00000001, lsb = 01010000
        // Smush them together with bit shifting to get 0b0000000101010000 == 0x150
        msb << 8 | lsb
    }

    pub fn push_to_stack(&mut self, value: u16) {

        self.registers.dec_sp();
        self.write_byte_to_memory(self.registers.stack_pointer, Util::get_msb(value));
        self.registers.dec_sp();
        self.write_byte_to_memory(self.registers.stack_pointer, Util::get_lsb(value));
    }

    pub fn pop_from_stack(&mut self) -> u16 {
        let lsb = self.read_byte_from_memory(self.registers.stack_pointer);
        self.registers.inc_sp();
        let msb = self.read_byte_from_memory(self.registers.stack_pointer);
        self.registers.inc_sp();

        Util::bytes_to_word(msb, lsb)
    }
}
