use crate::emu::Emu;
use crate::registers::register::{Register, get_arithmetic_reg_yyy};

pub fn or_n(emu: &mut Emu, opcode: u8) -> u8 {

    let register = get_arithmetic_reg_yyy(opcode);

    let byte = emu.get_reg(&register);
    or(emu, byte);

    if !matches!(register, Register::HLPOINTER) {
        return 2;
    }

    1
}

fn or(emu: &mut Emu, byte: u8) {
    emu.registers.accumulator = emu.registers.accumulator | byte;

    emu.registers.flags.zero = emu.registers.accumulator == 0;
    emu.registers.flags.negative = false;
    emu.registers.flags.half_carry = false;
    emu.registers.flags.carry = false;
}
