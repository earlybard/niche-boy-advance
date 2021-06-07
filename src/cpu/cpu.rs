use crate::emu::Emu;
use crate::cpu::instructions::jump::{jump_relative, jump_nn};
use crate::cpu::instructions::xor::xor;
use crate::cpu::instructions::load_u8::{load_r_n, load_rr, load_hli_a, load_hld_a, load_a_hli, load_a_hld, ldh_n_a, ldh_a_n, ldh_c_a, ldh_a_c, load_rr_a, load_a_rr};
use crate::cpu::instructions::misc::{di, noop};
use crate::cpu::instructions::compare::compare;
use crate::cpu::instructions::call_ret::{call, ret, restart};
use crate::registers::register::RegisterType::{A, B, H, D, HLPOINTER, C, E, L};
use crate::cpu::instructions::bitwise::res;
use crate::cpu::instructions::and::{and_u8, and_n};
use crate::cpu::instructions::inc_dec::{inc_rr, dec_nn, inc_r, dec_r};
use crate::cpu::instructions::or::or_n;
use crate::cpu::conditionals::Condition::{Unconditional, NotZero, Zero, NotCarry, Carry};
use std::process::exit;
use crate::registers::register::RegisterPairType::{BC, DE, HL, SP, AF};
use crate::cpu::instructions::stack::{push, pop};
use crate::cpu::instructions::load_u16::{load_rr_nn, load_nn_a, load_a_nn};
use crate::cpu::instructions::add_u16::add_hl_rr;
use crate::cpu::conditionals::Condition;


#[derive(Debug)]
#[derive(Default)]
pub struct CPU {
    pub halted: bool,
    pub m_cycles: u8
}

impl CPU {
    pub fn cycle(&mut self) {
        self.m_cycles += 1;
    }
}

impl Emu {
    pub fn run_operand(&mut self) {

        println!("PC: {:#6X?}", self.registers.program_counter);

        // Reset cycles for this operand.
        self.cpu.m_cycles = 0;

        let opcode = self.read_and_inc();

        println!("OP: {:#04X?}", opcode);
        println!("{:?}", &self.registers);
        println!("{:?}", &self.registers.flags);

        match opcode {
            0x00 => noop(),

            0x01 => load_rr_nn(self, BC),
            0x11 => load_rr_nn(self, DE),
            0x21 => load_rr_nn(self, HL),
            0x31 => load_rr_nn(self, SP),

            0x02 => load_rr_a(self, BC),
            0x12 => load_rr_a(self, DE),
            0x22 => load_hli_a(self),
            0x32 => load_hld_a(self),

            0x03 => inc_rr(self, BC),
            0x13 => inc_rr(self, DE),
            0x23 => inc_rr(self, HL),
            0x33 => inc_rr(self, SP),

            0x04 => inc_r(self, B),
            0x14 => inc_r(self, D),
            0x24 => inc_r(self, H),
            0x34 => inc_r(self, HLPOINTER),

            0x05 => dec_r(self, B),
            0x15 => dec_r(self, D),
            0x25 => dec_r(self, H),
            0x35 => dec_r(self, HLPOINTER),

            0x06 => load_r_n(self, B),
            0x16 => load_r_n(self, D),
            0x26 => load_r_n(self, H),
            0x36 => load_r_n(self, HLPOINTER),

            0x18 => jump_relative(self, Unconditional),
            0x20 => jump_relative(self, NotZero),
            0x28 => jump_relative(self, Zero),
            0x30 => jump_relative(self, NotCarry),
            0x38 => jump_relative(self, Carry),

            0x09 => add_hl_rr(self, BC),
            0x19 => add_hl_rr(self, DE),
            0x29 => add_hl_rr(self, HL),
            0x39 => add_hl_rr(self, SP),

            // TODO r is register n is immediate everywhere

            0x0A => load_a_rr(self, BC),
            0x1A => load_a_rr(self, DE),
            0x2A => load_a_hli(self),
            0x3A => load_a_hld(self),

            0x0E => load_r_n(self, C),
            0x1E => load_r_n(self, E),
            0x2E => load_r_n(self, L),
            0x3E => load_r_n(self, A),

            0x0B => dec_nn(self, BC),
            0x1B => dec_nn(self, DE),
            0x2B => dec_nn(self, HL),
            0x3B => dec_nn(self, SP),

            0x0C => inc_r(self, C),
            0x1C => inc_r(self, E),
            0x2C => inc_r(self, L),
            0x3C => inc_r(self, A),

            0x0D => dec_r(self, C),
            0x1D => dec_r(self, E),
            0x2D => dec_r(self, L),
            0x3D => dec_r(self, A),

            0x40..=0x7F => load_rr(self, opcode),
            0xA0..=0xA7 => and_n(self, opcode),
            0xB0..=0xB7 => or_n(self, opcode),
            0xAF => xor(self, A),
            0xC3 => jump_nn(self),
            0xCB => self.run_prefix(),

            0xC0 => ret(self, NotZero),
            0xD0 => ret(self, NotCarry),
            0xE0 => ldh_n_a(self),
            0xF0 => ldh_a_n(self),

            0xC1 => pop(self, BC),
            0xD1 => pop(self, DE),
            0xE1 => pop(self, HL),
            0xF1 => pop(self, AF),

            0xE2 => ldh_c_a(self),
            0xF2 => ldh_a_c(self),

            0xC4 => call(self, NotZero),
            0xD4 => call(self, NotCarry),
            0xCC => call(self, Zero),
            0xDC => call(self, Carry),
            0xCD => call(self, Unconditional),

            0xC5 => push(self, BC),
            0xD5 => push(self, DE),
            0xE5 => push(self, HL),
            0xF5 => push(self, AF),

            0xE6 => and_u8(self),

            0xC7 => restart(self, 00),
            0xD7 => restart(self, 10),
            0xE7 => restart(self, 20),
            0xF7 => restart(self, 30),

            0xC8 => ret(self, Zero),
            0xD8 => ret(self, Carry),

            0xC9 => ret(self, Unconditional),

            0xEA => load_nn_a(self),
            0xFA => load_a_nn(self),

            0xF3 => di(self),
            0xFE => compare(self),

            0xCF => restart(self, 08),
            0xDF => restart(self, 18),
            0xEF => restart(self, 28),
            0xFF => restart(self, 38),
            _ => {
                println!("Unknown opcode: {:#04X?}", opcode);
                println!("{:?}", &self.registers);
                println!("{:?}", &self.registers.flags);
                exit(0);
            }
        };
    }

    fn run_prefix(&mut self) {

        let opcode = self.read_and_inc();

        match opcode {
            0x87 => res(self, 0, A),
            _ => {
                println!("Unknown prefix opcode: {:#4X?}", opcode);
                println!("{:?}", &self.registers);
                println!("{:?}", &self.registers.flags);
                exit(0);
            }
        }
    }
}