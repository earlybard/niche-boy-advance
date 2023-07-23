use crate::emu::Emu;
use crate::registers::register::RegisterType;

pub fn swap(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);

    let mut upper_nibble = byte & 0b11110000;
    let mut lower_nibble = byte & 0b00001111;

    upper_nibble <<= 4;
    lower_nibble >>= 4;

    let output = upper_nibble | lower_nibble;

    emu.write_register(&register, output);
}