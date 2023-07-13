use crate::emu::Emu;
use crate::cpu::instructions::jump::{jump_relative, jump, jump_hl};
use crate::cpu::instructions::xor::xor;
use crate::cpu::instructions::load_u8::{load_rr_from_opcode, load_hli_a, load_hld_a, load_a_hli, load_a_hld, ldh_n_a, ldh_a_n, ldh_c_a, ldh_a_c, load_rr_a, load_a_rr, load_r_r};
use crate::cpu::instructions::misc::{ccf, cpl, di, noop};
use crate::cpu::instructions::compare::{compare};
use crate::cpu::instructions::call_ret::{call, ret, restart};
use crate::registers::register::RegisterType::{A, B, H, D, HLPOINTER, C, E, L, NextU8};
use crate::cpu::instructions::bitwise::{reset, set, bit};
use crate::cpu::instructions::and::{and};
use crate::cpu::instructions::inc_dec::{inc_rr, dec_rr, inc_r, dec_r};
use crate::cpu::instructions::or::or;
use crate::cpu::conditionals::Condition::{Unconditional, NotZero, Zero, NotCarry, Carry};
use std::process::exit;
use crate::registers::register::RegisterPairType::{BC, DE, HL, SP, AF};
use crate::cpu::instructions::stack::{push, pop};
use crate::cpu::instructions::load_u16::{load_rr_nn, load_nn_a, load_a_nn, load_nn_sp, load_sp_hl};
use crate::cpu::instructions::add_u16::add_hl_rr;
use crate::cpu::instructions::arithmetic::{add, adc, sub, sbc};
use crate::cpu::instructions::rotate::{rl, rla, rlc, rlca, rr, rrc};
use crate::registers::register::RegisterType;

#[derive(Debug)]
#[derive(Default)]
pub struct CPU {
    pub halted: bool,
    pub m_cycles: u8
}

impl Emu {
    pub fn run_operand(&mut self) {

        // println!("PC: {:#6X?}", self.registers.program_counter);

        // Reset cycles for this operand.
        self.cpu.m_cycles = 0;

        // println!("{:#04X?}", self.registers.program_counter);
        // println!("{:?}", &self.registers);

        let opcode = self.read_pc();

        // println!("OP: {:#04X?}", opcode);
        // println!("{:?}", &self.registers.flags);

        match opcode {
            0x00 => noop(),

            0x20 => jump_relative(self, NotZero),
            0x30 => jump_relative(self, NotCarry),

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

            0x06 => load_r_r(self, NextU8, B),
            0x16 => load_r_r(self, NextU8, D),
            0x26 => load_r_r(self, NextU8, H),
            0x36 => load_r_r(self, NextU8, HLPOINTER),

            0x07 => rlca(self),
            0x17 => rla(self),

            0x08 => load_nn_sp(self),
            0x18 => jump_relative(self, Unconditional),
            0x28 => jump_relative(self, Zero),
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

            0x0E => load_r_r(self, NextU8, C),
            0x1E => load_r_r(self, NextU8, E),
            0x2E => load_r_r(self, NextU8, L),
            0x3E => load_r_r(self, NextU8, A),

            0x0B => dec_rr(self, BC),
            0x1B => dec_rr(self, DE),
            0x2B => dec_rr(self, HL),
            0x3B => dec_rr(self, SP),

            0x0C => inc_r(self, C),
            0x1C => inc_r(self, E),
            0x2C => inc_r(self, L),
            0x3C => inc_r(self, A),

            0x0D => dec_r(self, C),
            0x1D => dec_r(self, E),
            0x2D => dec_r(self, L),
            0x3D => dec_r(self, A),

            0x0F => rrc(self, A),
            0x1F => rr(self, A),
            0x2F => cpl(self),
            0x3F => ccf(self),

            0x40..=0x7F => load_rr_from_opcode(self, opcode),

            0x80 => add(self, B),
            0x81 => add(self, C),
            0x82 => add(self, D),
            0x83 => add(self, E),
            0x84 => add(self, H),
            0x85 => add(self, L),
            0x86 => add(self, HLPOINTER),
            0x87 => add(self, A),

            0x88 => adc(self, B),
            0x89 => adc(self, C),
            0x8A => adc(self, D),
            0x8B => adc(self, E),
            0x8C => adc(self, H),
            0x8D => adc(self, L),
            0x8E => adc(self, HLPOINTER),
            0x8F => adc(self, A),

            0x90 => sub(self, B),
            0x91 => sub(self, C),
            0x92 => sub(self, D),
            0x93 => sub(self, E),
            0x94 => sub(self, H),
            0x95 => sub(self, L),
            0x96 => sub(self, HLPOINTER),
            0x97 => sub(self, A),

            0x98 => sbc(self, B),
            0x99 => sbc(self, C),
            0x9A => sbc(self, D),
            0x9B => sbc(self, E),
            0x9C => sbc(self, H),
            0x9D => sbc(self, L),
            0x9E => sbc(self, HLPOINTER),
            0x9F => sbc(self, A),

            0xA0 => and(self, B),
            0xA1 => and(self, C),
            0xA2 => and(self, D),
            0xA3 => and(self, E),
            0xA4 => and(self, H),
            0xA5 => and(self, L),
            0xA6 => and(self, HLPOINTER),
            0xA7 => and(self, A),

            0xA8 => xor(self, B),
            0xA9 => xor(self, C),
            0xAA => xor(self, D),
            0xAB => xor(self, E),
            0xAC => xor(self, H),
            0xAD => xor(self, L),
            0xAE => xor(self, HLPOINTER),
            0xAF => xor(self, A),

            0xB0 => or(self, B),
            0xB1 => or(self, C),
            0xB2 => or(self, D),
            0xB3 => or(self, E),
            0xB4 => or(self, H),
            0xB5 => or(self, L),
            0xB6 => or(self, HLPOINTER),
            0xB7 => or(self, A),

            0xB8 => compare(self, B),
            0xB9 => compare(self, C),
            0xBA => compare(self, D),
            0xBB => compare(self, E),
            0xBC => compare(self, H),
            0xBD => compare(self, L),
            0xBE => compare(self, HLPOINTER),
            0xBF => compare(self, A),

            0xC2 => jump(self, NotZero),
            0xD2 => jump(self, NotCarry),
            0xE2 => ldh_c_a(self),
            0xF2 => ldh_a_c(self),

            0xC3 => jump(self, Unconditional),
            0xD3 => noop(),
            0xE3 => noop(),
            0xF3 => di(self),

            0xC4 => call(self, NotZero),
            0xD4 => call(self, NotCarry),
            0xE4 => noop(),
            0xF4 => noop(),

            0xC5 => push(self, BC),
            0xD5 => push(self, DE),
            0xE5 => push(self, HL),
            0xF5 => push(self, AF),

            0xC6 => add(self, NextU8),
            0xD6 => sub(self, NextU8),
            0xE6 => and(self, NextU8),
            0xF6 => or(self, NextU8),

            0xC0 => ret(self, NotZero),
            0xD0 => ret(self, NotCarry),
            0xE0 => ldh_n_a(self),
            0xF0 => ldh_a_n(self),

            0xC1 => pop(self, BC),
            0xD1 => pop(self, DE),
            0xE1 => pop(self, HL),
            0xF1 => pop(self, AF),

            0xCC => call(self, Zero),
            0xDC => call(self, Carry),
            0xCD => call(self, Unconditional),

            0xC7 => restart(self, 00),
            0xD7 => restart(self, 10),
            0xE7 => restart(self, 20),
            0xF7 => restart(self, 30),

            0xC8 => ret(self, Zero),
            0xD8 => ret(self, Carry),

            0xC9 => ret(self, Unconditional),

            0xE9 => jump_hl(self),
            0xF9 => load_sp_hl(self),

            0xCA => jump(self, Zero),
            0xDA => jump(self, Carry),
            0xEA => load_nn_a(self),
            0xFA => load_a_nn(self),

            // 0x27 => {}, // daa
            // 0x37 => {}, // scf
            // 0xFB => {},
            0xCB => self.run_prefix(),

            0xCE => adc(self, NextU8),
            0xDE => sbc(self, NextU8),
            0xEE => xor(self, NextU8),
            0xFE => compare(self, NextU8),

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

        let opcode = self.read_pc();

        match opcode {

            0x00 => rlc(self, B),
            0x01 => rlc(self, C),
            0x02 => rlc(self, D),
            0x03 => rlc(self, E),
            0x04 => rlc(self, H),
            0x05 => rlc(self, L),
            0x06 => rlc(self, HLPOINTER),
            0x07 => rlc(self, A),

            0x08 => rrc(self, B),
            0x09 => rrc(self, C),
            0x0A => rrc(self, D),
            0x0B => rrc(self, E),
            0x0C => rrc(self, H),
            0x0D => rrc(self, L),
            0x0E => rrc(self, HLPOINTER),
            0x0F => rrc(self, A),

            0x10 => rl(self, B),
            0x11 => rl(self, C),
            0x12 => rl(self, D),
            0x13 => rl(self, E),
            0x14 => rl(self, H),
            0x15 => rl(self, L),
            0x16 => rl(self, HLPOINTER),
            0x17 => rl(self, A),

            0x18 => rr(self, B),
            0x19 => rr(self, C),
            0x1A => rr(self, D),
            0x1B => rr(self, E),
            0x1C => rr(self, H),
            0x1D => rr(self, L),
            0x1E => rr(self, HLPOINTER),
            0x1F => rr(self, A),

            0x40 => bit(self, 0, B),
            0x41 => bit(self, 0, C),
            0x42 => bit(self, 0, D),
            0x43 => bit(self, 0, E),
            0x44 => bit(self, 0, H),
            0x45 => bit(self, 0, L),
            0x46 => bit(self, 0, HLPOINTER),
            0x47 => bit(self, 0, A),

            0x48 => bit(self, 1, B),
            0x49 => bit(self, 1, C),
            0x4A => bit(self, 1, D),
            0x4B => bit(self, 1, E),
            0x4C => bit(self, 1, H),
            0x4D => bit(self, 1, L),
            0x4E => bit(self, 1, HLPOINTER),
            0x4F => bit(self, 1, A),

            0x50 => bit(self, 2, B),
            0x51 => bit(self, 2, C),
            0x52 => bit(self, 2, D),
            0x53 => bit(self, 2, E),
            0x54 => bit(self, 2, H),
            0x55 => bit(self, 2, L),
            0x56 => bit(self, 2, HLPOINTER),
            0x57 => bit(self, 2, A),

            0x58 => bit(self, 3, B),
            0x59 => bit(self, 3, C),
            0x5A => bit(self, 3, D),
            0x5B => bit(self, 3, E),
            0x5C => bit(self, 3, H),
            0x5D => bit(self, 3, L),
            0x5E => bit(self, 3, HLPOINTER),
            0x5F => bit(self, 3, A),

            0x60 => bit(self, 4, B),
            0x61 => bit(self, 4, C),
            0x62 => bit(self, 4, D),
            0x63 => bit(self, 4, E),
            0x64 => bit(self, 4, H),
            0x65 => bit(self, 4, L),
            0x66 => bit(self, 4, HLPOINTER),
            0x67 => bit(self, 4, A),

            0x68 => bit(self, 5, B),
            0x69 => bit(self, 5, C),
            0x6A => bit(self, 5, D),
            0x6B => bit(self, 5, E),
            0x6C => bit(self, 5, H),
            0x6D => bit(self, 5, L),
            0x6E => bit(self, 5, HLPOINTER),
            0x6F => bit(self, 5, A),

            0x70 => bit(self, 6, B),
            0x71 => bit(self, 6, C),
            0x72 => bit(self, 6, D),
            0x73 => bit(self, 6, E),
            0x74 => bit(self, 6, H),
            0x75 => bit(self, 6, L),
            0x76 => bit(self, 6, HLPOINTER),
            0x77 => bit(self, 6, A),

            0x78 => bit(self, 7, B),
            0x79 => bit(self, 7, C),
            0x7A => bit(self, 7, D),
            0x7B => bit(self, 7, E),
            0x7C => bit(self, 7, H),
            0x7D => bit(self, 7, L),
            0x7E => bit(self, 7, HLPOINTER),
            0x7F => bit(self, 7, A),

            0x80 => reset(self, 0, B),
            0x81 => reset(self, 0, C),
            0x82 => reset(self, 0, D),
            0x83 => reset(self, 0, E),
            0x84 => reset(self, 0, H),
            0x85 => reset(self, 0, L),
            0x86 => reset(self, 0, HLPOINTER),
            0x87 => reset(self, 0, A),

            0x88 => reset(self, 1, B),
            0x89 => reset(self, 1, C),
            0x8A => reset(self, 1, D),
            0x8B => reset(self, 1, E),
            0x8C => reset(self, 1, H),
            0x8D => reset(self, 1, L),
            0x8E => reset(self, 1, HLPOINTER),
            0x8F => reset(self, 1, A),

            0x90 => reset(self, 2, B),
            0x91 => reset(self, 2, C),
            0x92 => reset(self, 2, D),
            0x93 => reset(self, 2, E),
            0x94 => reset(self, 2, H),
            0x95 => reset(self, 2, L),
            0x96 => reset(self, 2, HLPOINTER),
            0x97 => reset(self, 2, A),

            0x98 => reset(self, 3, B),
            0x99 => reset(self, 3, C),
            0x9A => reset(self, 3, D),
            0x9B => reset(self, 3, E),
            0x9C => reset(self, 3, H),
            0x9D => reset(self, 3, L),
            0x9E => reset(self, 3, HLPOINTER),
            0x9F => reset(self, 3, A),

            0xA0 => reset(self, 4, B),
            0xA1 => reset(self, 4, C),
            0xA2 => reset(self, 4, D),
            0xA3 => reset(self, 4, E),
            0xA4 => reset(self, 4, H),
            0xA5 => reset(self, 4, L),
            0xA6 => reset(self, 4, HLPOINTER),
            0xA7 => reset(self, 4, A),

            0xA8 => reset(self, 5, B),
            0xA9 => reset(self, 5, C),
            0xAA => reset(self, 5, D),
            0xAB => reset(self, 5, E),
            0xAC => reset(self, 5, H),
            0xAD => reset(self, 5, L),
            0xAE => reset(self, 5, HLPOINTER),
            0xAF => reset(self, 5, A),

            0xB0 => reset(self, 6, B),
            0xB1 => reset(self, 6, C),
            0xB2 => reset(self, 6, D),
            0xB3 => reset(self, 6, E),
            0xB4 => reset(self, 6, H),
            0xB5 => reset(self, 6, L),
            0xB6 => reset(self, 6, HLPOINTER),
            0xB7 => reset(self, 6, A),

            0xB8 => reset(self, 7, B),
            0xB9 => reset(self, 7, C),
            0xBA => reset(self, 7, D),
            0xBB => reset(self, 7, E),
            0xBC => reset(self, 7, H),
            0xBD => reset(self, 7, L),
            0xBE => reset(self, 7, HLPOINTER),
            0xBF => reset(self, 7, A),


            0xCF => set(self, 1, A),
            0xFF => set(self, 7, A),
            _ => {
                println!("Unknown PREFIX opcode: {:#4X?}", opcode);
                println!("{:?}", &self.registers);
                println!("{:?}", &self.registers.flags);
                exit(0);
            }
        }
    }
}