use crate::cpu::cpu::Emu;
use crate::cpu::instructions::jump::{jump_relative, jump};
use crate::cpu::instructions::xor::xor;
use crate::cpu::register::Register;
use std::path::Prefix::UNC;
use crate::cpu::instructions::load::{load, load_to_register, load_control_to_register, load_register_to_register};
use crate::cpu::instructions::misc::{di, noop};
use crate::cpu::instructions::compare::compare;
use crate::cpu::instructions::jump::JumpRelativeCondition::{UNCONDITIONAL, Z, NZ};
use crate::cpu::instructions::load::LoadMode::{FF00, WORD};
use crate::cpu::instructions::call::call;
use crate::cpu::register::Register::{B, A};
use crate::cpu::instructions::res::res;
use crate::cpu::instructions::and::and_immediate;

pub struct OpCodes {}

impl OpCodes {
    pub fn run_op(&self, opcode: u8, cpu: &mut Emu) -> u8 {

        return match opcode {
            0x00 => noop(),
            0x18 => jump_relative(cpu, UNCONDITIONAL),
            0x20 => jump_relative(cpu, NZ),
            0x28 => jump_relative(cpu, Z),
            0x47 => load_register_to_register(cpu, A, B),
            0x3E => load_to_register(cpu, A),
            0xAF => xor(cpu, A),
            0xC3 => jump(cpu),
            0xCB => self.run_prefix(cpu),
            0xCD => call(cpu),
            0xE0 => load(cpu, FF00),
            0xE6 => and_immediate(cpu),
            0xEA => load(cpu, WORD),
            0xF0 => load_control_to_register(cpu, A),
            0xF3 => di(cpu),
            0xFE => compare(cpu),
            _ => 0
        }
    }

    fn run_prefix(&self, cpu: &mut Emu) -> u8 {

        let opcode = cpu.read_and_inc();

        return match opcode {
            0x87 => res(cpu, 0, A),
            _ => {
                println!("Unknown prefix opcode: {:#4X?}", opcode);
                0
            }
        }
    }
}
