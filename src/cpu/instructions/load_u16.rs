use crate::emu::Emu;
use crate::registers::register::RegisterPairType;

/// Load nn into the given register pair rr.
pub fn load_rr_nn(emu: &mut Emu, register_pair: RegisterPairType) {

    let word = emu.read_u16_and_inc();
    emu.write_register_pair(&register_pair, word);
}

/// Load (nn) into A.
pub fn load_a_nn(emu: &mut Emu) {
    let addr = emu.read_u16_and_inc();
    let value = emu.read_byte_from_memory(addr);
    emu.registers.accumulator = value;
}

/// Load A into (nn).
pub fn load_nn_a(emu: &mut Emu) {
    let addr = emu.read_u16_and_inc();
    let value = emu.registers.accumulator;
    emu.write_byte_to_memory(addr, value);
}

pub fn load_nn_sp(emu: &mut Emu) {
    let addr = emu.read_u16_and_inc();
    emu.write_word_to_memory(addr, emu.registers.stack_pointer)
}

pub fn load_sp_hl(emu: &mut Emu) {
    emu.cycle();
    emu.registers.stack_pointer = emu.registers.hl.get_word();
}
