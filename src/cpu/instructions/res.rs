use crate::emu::Emu;
use crate::registers::register::Register;
use crate::util::Util;

pub fn res(cpu: &mut Emu, bit: u8, register: Register) -> u8 {

    let val = cpu.get_reg(&register);

    let result = Util::reset_flag(val, bit);

    cpu.set_reg(&register, result);

    2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_res() {

        let mut cpu = Emu::default();

        cpu.registers.hl.first = 0b10111001;
        res(&mut cpu, 7, Register::H);
        assert_eq!(cpu.registers.hl.first, 0b00111001);

        res(&mut cpu, 4, Register::H);
        assert_eq!(cpu.registers.hl.first, 0b00101001);
    }
}