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

#[derive(Debug)]
pub enum RegisterPair {
    BC,
    DE,
    HL,
    SP
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

    pub fn get_reg_pair(&self, register_pair: &RegisterPair) -> u16 {
        return match register_pair {
            // RegisterPair::AF => self.registers.get_af(),
            RegisterPair::BC => self.registers.bc.get_word(),
            RegisterPair::DE => self.registers.de.get_word(),
            RegisterPair::HL => self.registers.hl.get_word(),
            RegisterPair::SP => self.registers.sp
        }
    }

    pub fn set_reg_pair(&mut self, register_pair: &RegisterPair, value: u16) {
        match register_pair {
            // RegisterPair::AF => todo!("Setter function for AF"),
            RegisterPair::BC => self.registers.bc.set_word(value),
            RegisterPair::DE => self.registers.de.set_word(value),
            RegisterPair::HL => self.registers.hl.set_word(value),
            RegisterPair::SP => self.registers.sp = value
        }
    }
}

fn get_arithmetic_reg(code: u8) -> Register {
    return match code {
        0 => Register::B,
        1 => Register::C,
        2 => Register::D,
        3 => Register::E,
        4 => Register::H,
        5 => Register::L,
        6 => Register::HLPOINTER,
        7 => Register::A,
        _ => panic!(format!("Not a valid register: {}", code))
    }
}

pub fn get_arithmetic_reg_xxx(opcode: u8) -> Register {
    let xxx = (opcode & 0b00111000) >> 3;
    get_arithmetic_reg(xxx)
}

pub fn get_arithmetic_reg_yyy(opcode: u8) -> Register {
    let yyy = opcode & 0b00000111;
    get_arithmetic_reg(yyy)
}

fn get_arithmetic_reg_pair(code: u8) -> RegisterPair {
    return match code {
        0 => RegisterPair::BC,
        1 => RegisterPair::DE,
        2 => RegisterPair::HL,
        3 => RegisterPair::SP,
        _ => panic!(format!("Not a valid register pair: {}", code))
    }
}

pub fn get_arithmetic_reg_pair_xx(opcode: u8) -> RegisterPair {
    let xx = (opcode & 0b00110000) >> 4;
    get_arithmetic_reg_pair(xx)
}