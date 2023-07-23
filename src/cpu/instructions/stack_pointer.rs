use crate::emu::Emu;

pub fn add_n_to_sp(emu: &mut Emu) {
    let byte = emu.read_pc();
    let byte = byte as i16;
    let byte = byte - 128;

    // Now it's between -127 and 127
    if byte > 0 {
        emu.registers.stack_pointer = emu.registers.stack_pointer.wrapping_add(byte as u16);
    } else {
        emu.registers.stack_pointer = emu.registers.stack_pointer.wrapping_sub(byte.abs() as u16);
    }

    emu.registers.flags.zero = false;
    emu.registers.flags.negative = false;

    // These are wrong.
    emu.registers.flags.carry = false;
    emu.registers.flags.half_carry = false;
}

pub fn ld_hl_plus_n(emu: &mut Emu) {
    let mut sp = emu.registers.stack_pointer;

    let byte = emu.read_pc();
    let byte = byte as i16;
    let byte = byte - 128;

    if byte > 0 {
        sp = sp.wrapping_add(byte as u16);
    } else {
        sp = sp.wrapping_sub(byte.abs() as u16);
    }

    emu.registers.hl.set_word(sp);


    emu.registers.flags.zero = false;
    emu.registers.flags.negative = false;

    // These are wrong.
    emu.registers.flags.carry = false;
    emu.registers.flags.half_carry = false;
}