use crate::emu::Emu;
use crate::registers::register::{RegisterPair, get_arithmetic_reg_pair_xx};

pub fn inc_nn(emu: &mut Emu, opcode: u8) -> u8 {

    let register_pair = get_arithmetic_reg_pair_xx(opcode);
    let value = emu.get_reg_pair(&register_pair).wrapping_add(1);

    emu.set_reg_pair(&register_pair, value);
    2
}

pub fn dec_nn(emu: &mut Emu, opcode: u8) -> u8 {

    let register_pair = get_arithmetic_reg_pair_xx(opcode);
    let value = emu.get_reg_pair(&register_pair).wrapping_sub(1);

    emu.set_reg_pair(&register_pair, value);
    2
}
