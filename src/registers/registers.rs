use crate::registers::flags::Flags;
use crate::registers::register_pair::RegisterPair;
use std::fmt::{Debug, Formatter, Result};
use crate::util::Util;

#[derive(Default)]
pub struct Registers {
    pub accumulator: u8,
    pub flags: Flags,
    pub bc: RegisterPair,
    pub de: RegisterPair,
    pub hl: RegisterPair,
    pub stack_pointer: u16,
    pub program_counter: u16,
}

impl Debug for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Registers")
         .field("af", &format!("{:#06X?}", &self.get_af()))
         .field("bc", &format!("{:#06X?}", &self.bc.get_word()))
         .field("de", &format!("{:#06X?}", &self.de.get_word()))
         .field("hl", &format!("{:#06X?}", &self.hl.get_word()))
         .field("pc", &format!("{:#06X?}", &self.program_counter))
         .finish()
    }
}

impl Registers {
    pub fn get_af(&self) -> u16 {
        Util::bytes_to_word(self.accumulator, self.flags.get_byte())
    }
    // pub fn set_af(&mut self, value: u16) {
    //     todo!();
    // }
    pub fn dec_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_sub(1)
    }
    pub fn inc_sp(&mut self) {
        self.stack_pointer = self.stack_pointer.wrapping_add(1)
    }
}
