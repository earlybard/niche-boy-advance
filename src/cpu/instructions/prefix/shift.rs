use crate::emu::Emu;
use crate::registers::register::RegisterType;
use crate::util::{LSB, MSB, Util};

/// Shift left into carry.
pub fn sla(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);

    let msb = Util::get_bit(byte, MSB);

    let output = byte << 1;

    emu.registers.flags.carry = msb;
    emu.registers.flags.half_carry = false;
    emu.registers.flags.negative = false;
    emu.registers.flags.zero = output == 0;

    emu.write_register(&register, output);
}

/// Shift right into carry. Arithmetic: Keep MSB the same.
pub fn sra(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);

    let msb = Util::get_bit(byte, MSB);
    let lsb = Util::get_bit(byte, LSB);

    let mut output = byte >> 1;

    if msb {
        Util::set_bit(output, MSB);
    } else {
        Util::reset_bit(output, MSB);
    }

    emu.registers.flags.carry = lsb;
    emu.registers.flags.half_carry = false;
    emu.registers.flags.negative = false;
    emu.registers.flags.zero = output == 0;

    emu.write_register(&register, output);
}

/// Shift right into carry. Logical: MSB becomes 0.
pub fn srl(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);
    let lsb = Util::get_bit(byte, LSB);

    let output = byte >> 1;

    emu.registers.flags.carry = lsb;
    emu.registers.flags.half_carry = false;
    emu.registers.flags.negative = false;
    emu.registers.flags.zero = output == 0;

    emu.write_register(&register, output);
}