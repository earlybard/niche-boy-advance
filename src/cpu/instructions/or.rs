use crate::emu::Emu;
use crate::registers::register::{get_arithmetic_reg_yyy, RegisterType};

pub fn or(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);
    or_internal(emu, byte);
}

fn or_internal(emu: &mut Emu, byte: u8) {
    emu.registers.accumulator = emu.registers.accumulator | byte;

    emu.registers.flags.zero = emu.registers.accumulator == 0;
    emu.registers.flags.negative = false;
    emu.registers.flags.half_carry = false;
    emu.registers.flags.carry = false;
}
