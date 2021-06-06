use crate::emu::Emu;
use crate::registers::register::RegisterPair;

pub fn push(emu: &mut Emu, register_pair: RegisterPair) -> u8 {

    let value = emu.read_u16_and_inc();
    emu.push_to_stack(value);

    4
}

pub fn pop(emu: &mut Emu, register_pair: RegisterPair) -> u8 {

    let value = emu.read_u16_and_inc();
    emu.push_to_stack(value);

    4
}
