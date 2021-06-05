use crate::emu::Emu;
use crate::cpu::instructions::jump::{jump_relative, jump};
use crate::cpu::instructions::xor::xor;
use crate::registers::register::Register;
use std::path::Prefix::UNC;
use crate::cpu::instructions::load::{load, load_to_register, load_control_to_register, load_rr};
use crate::cpu::instructions::misc::{di, noop};
use crate::cpu::instructions::compare::compare;
use crate::cpu::instructions::jump::JumpRelativeCondition::{UNCONDITIONAL, Z, NZ};
use crate::cpu::instructions::load::LoadMode::{FF00, WORD};
use crate::cpu::instructions::call::{call, ret};
use crate::registers::register::Register::{B, A};
use crate::cpu::instructions::res::res;
use crate::cpu::instructions::and::and_immediate;


#[derive(Debug)]
#[derive(Default)]
pub struct CPU {
    pub halted: bool
}

impl Emu {
    pub fn run_operand(&mut self, opcode: u8) -> u8 {

        return match opcode {
            0x00 => noop(),
            0x18 => jump_relative(self, UNCONDITIONAL),
            0x20 => jump_relative(self, NZ),
            0x28 => jump_relative(self, Z),
            0x40..=0x7F => load_rr(self, opcode),
            0x3E => load_to_register(self, A),
            0xAF => xor(self, A),
            0xC3 => jump(self),
            0xC9 => ret(self),
            0xCB => self.run_prefix(),
            0xCD => call(self),
            0xE0 => load(self, FF00),
            0xE6 => and_immediate(self),
            0xEA => load(self, WORD),
            0xF0 => load_control_to_register(self, A),
            0xF3 => di(self),
            0xFE => compare(self),
            _ => 0
        }
    }

    fn run_prefix(&mut self) -> u8 {

        let opcode = self.read_and_inc();

        return match opcode {
            0x87 => res(self, 0, A),
            _ => {
                println!("Unknown prefix opcode: {:#4X?}", opcode);
                0
            }
        }
    }
}