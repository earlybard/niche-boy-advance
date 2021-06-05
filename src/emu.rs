use std::fmt::{Debug, Formatter, Result};
use crate::util::Util;
use crate::registers::register::Register;
use crate::gpu::gpu::GPU;
use crate::cpu::cpu::CPU;
use crate::registers::flags::Flags;
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
    pub fn read(&self) -> u8 {
        self.memory.buffer[self.registers.pc as usize]
    }

    pub fn read_and_inc(&mut self) -> u8 {
        self.registers.pc += 1;
        self.memory.buffer[(self.registers.pc - 1) as usize]
    }

    pub fn read_u16_and_inc(&mut self) -> u16 {
        let lsb = self.read_and_inc() as u16;
        let msb = self.read_and_inc() as u16;

        // msb = 00000001, lsb = 01010000
        // Smush them together with bit shifting to get 0b0000000101010000 == 0x150
        msb << 8 | lsb
    }

    pub fn inc_pc(&mut self) {
        self.registers.pc += 1;
    }

    pub fn push_to_stack(&mut self, value: u16) {

        self.registers.dec_sp();
        self.memory.write_byte(self.registers.sp, Util::get_msb(value));
        self.registers.dec_sp();
        self.memory.write_byte(self.registers.sp, Util::get_lsb(value));
    }

    pub fn pop_from_stack(&mut self) -> u16 {
        let lsb = self.memory.read_byte(self.registers.sp);
        self.registers.inc_sp();
        let msb = self.memory.read_byte(self.registers.sp);
        self.registers.inc_sp();

        Util::bytes_to_word(msb, lsb)
    }
}
