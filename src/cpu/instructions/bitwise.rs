use crate::emu::Emu;
use crate::registers::register::RegisterType;
use crate::util::Util;

pub fn bit(emu: &mut Emu, bit: u8, register: RegisterType) {
    let byte = emu.read_register(&register);
    let flag = Util::get_bit(byte, bit);

    emu.registers.flags.zero = !flag;
    emu.registers.flags.negative = false;
    emu.registers.flags.half_carry = true;
}

pub fn set(emu: &mut Emu, bit: u8, register: RegisterType) {
    let byte = emu.read_register(&register);
    let value = Util::set_bit(byte, bit);
    emu.write_register(&register, value);
}

pub fn reset(emu: &mut Emu, bit: u8, register: RegisterType) {
    let byte = emu.read_register(&register);
    let value = Util::reset_bit(byte, bit);
    emu.write_register(&register, value);
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_res() {

        let mut cpu = Emu::default();

        cpu.registers.hl.first = 0b10111001;
        reset(&mut cpu, 7, RegisterType::H);
        assert_eq!(cpu.registers.hl.first, 0b00111001);

        reset(&mut cpu, 4, RegisterType::H);
        assert_eq!(cpu.registers.hl.first, 0b00101001);
    }
}
