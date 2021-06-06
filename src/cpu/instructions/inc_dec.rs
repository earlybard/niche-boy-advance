use crate::emu::Emu;
use crate::registers::register::{RegisterType, RegisterPairType};

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
    let value = emu.read_register(&register).wrapping_add(1);
    emu.write_register(&register, value);
}

pub fn dec_r(emu: &mut Emu, register: RegisterType) {
    let value = emu.read_register(&register).wrapping_sub(1);
    emu.write_register(&register, value);
}