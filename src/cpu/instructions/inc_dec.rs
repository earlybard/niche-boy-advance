use crate::emu::Emu;
use crate::registers::register::{RegisterType, RegisterPairType};
use crate::util::Util;

/// u16 inc/dec

pub fn inc_rr(emu: &mut Emu, register_pair: RegisterPairType) {

    inc_nn_nocycle(emu, register_pair);
    emu.cpu.cycle();
}

pub fn inc_nn_nocycle(emu: &mut Emu, register_pair: RegisterPairType) {

    let value = emu.read_register_pair(&register_pair).wrapping_add(1);
    emu.write_register_pair(&register_pair, value);
}

pub fn dec_nn(emu: &mut Emu, register_pair: RegisterPairType) {

    dec_nn_nocycle(emu, register_pair);
    emu.cpu.cycle();
}

pub fn dec_nn_nocycle(emu: &mut Emu, register_pair: RegisterPairType) {

    let value = emu.read_register_pair(&register_pair).wrapping_sub(1);
    emu.write_register_pair(&register_pair, value);
}

/// u8 inc/dec
pub fn inc_r(emu: &mut Emu, register: RegisterType) {

    let left = emu.read_register(&register);
    let (value, hc, _) = Util::add_with_carry_flags(left, 1);

    emu.write_register(&register, value);

    emu.registers.flags.zero = value == 0;
    emu.registers.flags.negative = false;
    emu.registers.flags.half_carry = hc;
}

pub fn dec_r(emu: &mut Emu, register: RegisterType) {

    let left = emu.read_register(&register);
    let (value, hc, _) = Util::sub_with_carry_flags(left, 1);

    emu.write_register(&register, value);

    emu.registers.flags.zero = value == 0;
    emu.registers.flags.negative = false;
    emu.registers.flags.half_carry = hc;
}
