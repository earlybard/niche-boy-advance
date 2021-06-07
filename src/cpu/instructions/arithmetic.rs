use crate::emu::Emu;
use crate::registers::register::RegisterType;
use crate::util::Util;

pub fn adc(emu: &mut Emu, register: RegisterType) {
    let mut byte = emu.read_register(&register);

    if emu.registers.flags.carry {
        byte = byte.wrapping_add(1);
    }

    emu.registers.accumulator = add_internal(emu, byte);
}

pub fn add(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);
    emu.registers.accumulator = add_internal(emu, byte);
}

fn add_internal(emu: &mut Emu, byte: u8) -> u8 {
    let (value, hc, c) = Util::add_with_flags(emu.registers.accumulator, byte);

    emu.registers.flags.zero = value == 0;
    emu.registers.flags.negative = false;
    emu.registers.flags.carry = c;
    emu.registers.flags.half_carry = hc;

    value
}

pub fn sub(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);
    emu.registers.accumulator = sub_internal(emu, byte);
}

pub fn sbc(emu: &mut Emu, register: RegisterType) {
    let mut byte = emu.read_register(&register);

    if emu.registers.flags.carry {
        byte = byte.wrapping_add(1);
    }

    emu.registers.accumulator = sub_internal(emu, byte);
}

fn sub_internal(emu: &mut Emu, byte: u8) -> u8 {
    let (value, hc, c) = Util::sub_with_flags(emu.registers.accumulator, byte);

    emu.registers.flags.zero = value == 0;
    emu.registers.flags.negative = true;
    emu.registers.flags.carry = c;
    emu.registers.flags.half_carry = hc;

    value
}
