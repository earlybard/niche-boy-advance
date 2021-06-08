use crate::emu::{Emu};
use crate::registers::register::{RegisterType, get_arithmetic_reg_xxx, get_arithmetic_reg_yyy, RegisterPairType};
use crate::registers::register::RegisterType::{HLPOINTER, A};
use crate::registers::register::RegisterPairType::{HL};
use crate::cpu::instructions::inc_dec::{inc_nn_nocycle, dec_nn_nocycle};

/// Load to register r from immediate u8 value n.
/// TODO this can just be load_r_r now.
pub fn load_r_n(cpu: &mut Emu, register: RegisterType) {

    let value = cpu.read_and_inc();
    cpu.write_register(&register, value);
}

// TODO remove
pub fn load_rr(cpu: &mut Emu, opcode: u8) {

    let to = get_arithmetic_reg_xxx(opcode);
    let from = get_arithmetic_reg_yyy(opcode);

    load_r_r(cpu, to, from);
}

/// Load to Register r from Register r.
pub fn load_r_r(emu: &mut Emu, to: RegisterType, from: RegisterType) {

    // TODO move halt.
    if matches!(from, HLPOINTER) && matches!(to, HLPOINTER) {
        emu.cpu.halted = true;
        return;
    }

    let value = emu.read_register(&from);
    emu.write_register(&to, value);
}

/// Load A into (rr).
pub fn load_rr_a(emu: &mut Emu, register_pair: RegisterPairType) {
    let addr = emu.read_register_pair(&register_pair);
    emu.write_byte_to_memory(addr, emu.registers.accumulator);
}

/// Load (rr) into A.
pub fn load_a_rr(emu: &mut Emu, register_pair: RegisterPairType) {
    let addr = emu.read_register_pair(&register_pair);
    emu.registers.accumulator = emu.read_byte_from_memory(addr);
}

/// Load (HL) into A, then decrement HL.
pub fn load_a_hld(emu: &mut Emu) {
    load_r_r(emu, A, HLPOINTER);
    dec_nn_nocycle(emu, HL);
}

/// Load A into (HL), then decrement HL.
pub fn load_hld_a(emu: &mut Emu) {
    load_r_r(emu, HLPOINTER, A);
    dec_nn_nocycle(emu, HL);
}

/// Load (HL) into A, then increment HL.
pub fn load_a_hli(emu: &mut Emu) {
    load_r_r(emu, A, HLPOINTER);
    inc_nn_nocycle(emu, HL);
}

/// Load A into (HL), then increment HL.
pub fn load_hli_a(emu: &mut Emu) {
    load_r_r(emu, HLPOINTER, A);
    inc_nn_nocycle(emu, HL);
}

/// Load A into (0xFF00+n).
pub fn ldh_n_a(emu: &mut Emu) {
    let addr = 0xFF00 + (emu.read_and_inc() as u16);
    let value = emu.registers.accumulator;
    emu.write_byte_to_memory(addr, value);
}

/// Load (0xFF00+n) into A.
pub fn ldh_a_n(emu: &mut Emu) {
    let addr = 0xFF00 + (emu.read_and_inc() as u16);
    let value = emu.read_byte_from_memory(addr);
    emu.registers.accumulator = value;
}

/// Load A into (0xFF00+C).
pub fn ldh_c_a(emu: &mut Emu) {
    let addr = 0xFF00 + (emu.registers.bc.second as u16);
    let value = emu.registers.accumulator;
    emu.write_byte_to_memory(addr, value);
}

/// Load (0xFF00+C) into A.
pub fn ldh_a_c(emu: &mut Emu) {
    let addr = 0xFF00 + (emu.registers.bc.second as u16);
    let value = emu.read_byte_from_memory(addr);
    emu.registers.accumulator = value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::register::RegisterType::{D, E};

    #[test]
    fn test_load_hli_a() {
        let mut emu = Emu::default();
        emu.registers.accumulator = 5;

        load_hli_a(&mut emu);

        assert_eq!(emu.cpu.m_cycles, 1);
        assert_eq!(emu.registers.hl.get_word(), 1);
        assert_eq!(emu.memory.buffer[0], 5)
    }

    #[test]
    fn test_load_r_r() {

        let mut emu = Emu::default();
        emu.registers.de.first = 0x10;
        emu.registers.de.second = 0x20;

        // LD D, E
        load_r_r(&mut emu, D, E);

        assert_eq!(emu.registers.de.first, 0x20);
        assert_eq!(emu.registers.de.second, 0x20);

        emu.registers.de.first = 0x10;
        emu.registers.de.second = 0x20;

        // Test from opcode.
        load_rr(&mut emu, 0x53);

        assert_eq!(emu.registers.de.first, 0x20);
        assert_eq!(emu.registers.de.second, 0x20);
    }

    #[test]
    fn test_load_r_r_hlpointer() {
        let mut emu = Emu::default();

        emu.registers.de.first = 20;
        emu.registers.hl.set_word(0xFFAA);
        emu.memory.buffer[0xFFAA] = 10;

        load_r_r(&mut emu, HLPOINTER, D);

        assert_eq!(emu.memory.buffer[0xFFAA], 20);
        assert_eq!(emu.cpu.m_cycles, 1);
    }
}
