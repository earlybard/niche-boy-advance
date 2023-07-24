use minifb::Key::P;
use crate::emu::Emu;
use crate::util::Util;

pub fn add_n_to_sp(emu: &mut Emu) {
    let byte = emu.read_pc() as i8 as i16 as u16;

    let (result, half_carry, carry) =
        Util::add_u16_lower_with_flags(emu.registers.stack_pointer, byte);

    // Wrapping add.
    emu.registers.stack_pointer = result;

    emu.registers.flags.zero = false;
    emu.registers.flags.negative = false;
    emu.registers.flags.carry = carry;
    emu.registers.flags.half_carry = half_carry;
}

pub fn ld_hl_plus_n(emu: &mut Emu) {

    let byte = emu.read_pc() as i8 as i16 as u16;

    let (result, half_carry, carry)
        = Util::add_u16_lower_with_flags(emu.registers.stack_pointer, byte);

    emu.registers.hl.set_word(result);

    emu.registers.flags.zero = false;
    emu.registers.flags.negative = false;
    emu.registers.flags.carry = carry;
    emu.registers.flags.half_carry = half_carry;
}