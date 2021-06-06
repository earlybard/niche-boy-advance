use crate::emu::Emu;
use crate::registers::register::RegisterPairType;

pub fn push(emu: &mut Emu, register_pair: RegisterPairType) {

    let value = emu.read_register_pair(&register_pair);
    emu.cpu.cycle();
    emu.push_to_stack(value);
}

pub fn pop(emu: &mut Emu, register_pair: RegisterPairType) {

    let value = emu.pop_from_stack();
    emu.write_register_pair(&register_pair, value);
}
