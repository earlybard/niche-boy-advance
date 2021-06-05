use crate::emu::Emu;
use crate::util::Util;

#[derive(Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    HLPOINTER
}

impl Emu {
    pub fn get_reg(&self, register: &Register) -> u8 {
        return match register {
            Register::A => self.registers.accumulator,
            Register::B => self.registers.bc.first,
            Register::C => self.registers.bc.second,
            Register::D => self.registers.de.first,
            Register::E => self.registers.de.second,
            Register::F => self.registers.flags.get_byte(),
            Register::H => self.registers.hl.first,
            Register::L => self.registers.hl.second,
            Register::HLPOINTER => {
                // TODO add the extra cycle taken here.
                let word = self.registers.hl.get_word();
                self.memory.read_byte(word)
            }
        }
    }
    pub fn set_reg(&mut self, register: &Register, value: u8) {
        match register {
            Register::A => self.registers.accumulator = value,
            Register::B => self.registers.bc.first = value,
            Register::C => self.registers.bc.second = value,
            Register::D => self.registers.de.first = value,
            Register::E => self.registers.de.second = value,
            Register::F => todo!(),
            Register::H => self.registers.hl.first = value,
            Register::L => self.registers.hl.second = value,
            Register::HLPOINTER => {
                self.memory.write_byte(self.registers.hl.get_word(), value)
            }
        }
    }
}
