use std::fmt::{Debug, Formatter, Result};
use crate::util::Util;
use crate::cpu::register::Register;
use crate::cpu::interrupts::Interrupts;
use crate::gpu::gpu::GPU;

#[derive(Debug)]
#[derive(Default)]
pub struct Emu {
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
}

#[derive(Debug)]
#[derive(Default)]
pub struct Reg {
    pub first: u8,
    pub second: u8
}

impl Reg {
    fn get_word(&self) -> u16 {
        // Util::print_binary_u8(self.hi);
        // Util::print_binary_u8(self.lo);
        Util::bytes_to_word(self.first, self.second)
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct Flags {
    pub zero: bool,
    pub subtraction: bool,
    pub half_carry: bool,
    pub carry: bool,

    // These 4 bits always read 0 even if written with a 1.
    // _3: bool,
    // _2: bool,
    // _1: bool,
    // _0: bool
}

impl Flags {
    fn get_byte(&self) -> u8 {

        let mut result = 0u8;

        if self.zero { result = result | 0b10000000 };
        if self.subtraction { result = result | 0b01000000 };
        if self.half_carry { result = result | 0b00100000 };
        if self.carry { result = result | 0b00010000 };

        result
    }

    pub fn reset(&mut self) {
        self.zero = false;
        self.subtraction = false;
        self.half_carry = false;
        self.carry = false;
    }
}

#[derive(Default)]
pub struct Registers {
    pub accumulator: u8,
    pub flags: Flags,
    pub bc: Reg,
    pub de: Reg,
    pub hl: Reg,
    pub sp: u16,
    pub pc: u16,
}

impl Debug for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Registers")
            .field("af", &format!("{:#06X?}", &self.get_af()))
            .field("bc", &format!("{:#06X?}", &self.bc.get_word()))
            .field("de", &format!("{:#06X?}", &self.de.get_word()))
            .field("hl", &format!("{:#06X?}", &self.hl.get_word()))
            .field("pc", &format!("{:#06X?}", &self.pc))
            .finish()
    }
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        Util::bytes_to_word(self.accumulator, self.flags.get_byte())
    }
    pub fn get_value(&self, register: &Register) -> u8 {
        return match register {
            Register::A => self.accumulator,
            Register::B => self.bc.first,
            Register::C => self.bc.second,
            Register::D => self.de.first,
            Register::E => self.de.second,
            Register::F => self.flags.get_byte(),
            Register::H => self.hl.first,
            Register::L => self.hl.second,
        }
    }
    pub fn set_value(&mut self, register: &Register, value: u8) {
        match register {
            Register::A => self.accumulator = value,
            Register::B => self.bc.first = value,
            Register::C => self.bc.second = value,
            Register::D => self.de.first = value,
            Register::E => self.de.second = value,
            Register::F => todo!(),
            Register::H => self.hl.first = value,
            Register::L => self.hl.second = value
        }
    }
    pub fn dec_sp(&mut self) {
        self.sp = self.sp.wrapping_sub(1)
    }
    pub fn inc_sp(&mut self) {
        self.sp = self.sp.wrapping_add(1)
    }
}

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
