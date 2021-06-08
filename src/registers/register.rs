use crate::emu::Emu;
use crate::util::Util;

#[derive(Debug)]
pub enum RegisterType {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    HLPOINTER,
    NextU8
}

#[derive(Debug)]
pub enum RegisterPairType {
    BC,
    DE,
    HL,
    SP,
    AF
}

impl Emu {
    pub fn read_register(&mut self, register: &RegisterType) -> u8 {
        return match register {
            RegisterType::A => self.registers.accumulator,
            RegisterType::B => self.registers.bc.first,
            RegisterType::C => self.registers.bc.second,
            RegisterType::D => self.registers.de.first,
            RegisterType::E => self.registers.de.second,
            RegisterType::F => self.registers.flags.get_byte(),
            RegisterType::H => self.registers.hl.first,
            RegisterType::L => self.registers.hl.second,
            RegisterType::HLPOINTER => {
                let word = self.registers.hl.get_word();
                self.read_byte_from_memory(word)
            },
            RegisterType::NextU8 => self.read_and_inc()
        }
    }
    pub fn write_register(&mut self, register: &RegisterType, value: u8) {
        match register {
            RegisterType::A => self.registers.accumulator = value,
            RegisterType::B => self.registers.bc.first = value,
            RegisterType::C => self.registers.bc.second = value,
            RegisterType::D => self.registers.de.first = value,
            RegisterType::E => self.registers.de.second = value,
            RegisterType::F => todo!(),
            RegisterType::H => self.registers.hl.first = value,
            RegisterType::L => self.registers.hl.second = value,
            RegisterType::HLPOINTER => {
                self.write_byte_to_memory(self.registers.hl.get_word(), value)
            }
            RegisterType::NextU8 => panic!("Can't write to NextU8")
        }
    }

    pub fn read_register_pair(&self, register_pair: &RegisterPairType) -> u16 {
        return match register_pair {
            RegisterPairType::AF => self.registers.get_af(),
            RegisterPairType::BC => self.registers.bc.get_word(),
            RegisterPairType::DE => self.registers.de.get_word(),
            RegisterPairType::HL => self.registers.hl.get_word(),
            RegisterPairType::SP => self.registers.stack_pointer
        }
    }

    pub fn write_register_pair(&mut self, register_pair: &RegisterPairType, value: u16) {
        match register_pair {
            RegisterPairType::AF => {
                let (first, second) = Util::word_to_bytes(value);
                self.registers.accumulator = first;
                self.registers.flags.set_byte(second);
            }
            RegisterPairType::BC => self.registers.bc.set_word(value),
            RegisterPairType::DE => self.registers.de.set_word(value),
            RegisterPairType::HL => self.registers.hl.set_word(value),
            RegisterPairType::SP => self.registers.stack_pointer = value
        }
    }
}

fn get_arithmetic_reg(code: u8) -> RegisterType {
    return match code {
        0 => RegisterType::B,
        1 => RegisterType::C,
        2 => RegisterType::D,
        3 => RegisterType::E,
        4 => RegisterType::H,
        5 => RegisterType::L,
        6 => RegisterType::HLPOINTER,
        7 => RegisterType::A,
        _ => panic!("Not a valid register: {}", code)
    }
}

pub fn get_arithmetic_reg_xxx(opcode: u8) -> RegisterType {
    let xxx = (opcode & 0b00111000) >> 3;
    get_arithmetic_reg(xxx)
}

pub fn get_arithmetic_reg_yyy(opcode: u8) -> RegisterType {
    let yyy = opcode & 0b00000111;
    get_arithmetic_reg(yyy)
}

// fn get_arithmetic_reg_pair(code: u8) -> RegisterPairType {
//     return match code {
//         0 => RegisterPairType::BC,
//         1 => RegisterPairType::DE,
//         2 => RegisterPairType::HL,
//         3 => RegisterPairType::SP,
//         _ => panic!("Not a valid register pair: {}", code)
//     }
// }

// pub fn get_arithmetic_reg_pair_xx(opcode: u8) -> RegisterPairType {
//     let xx = (opcode & 0b00110000) >> 4;
//     get_arithmetic_reg_pair(xx)
// }