use crate::emu::Emu;
use crate::registers::register::{get_arithmetic_reg_yyy};

pub fn and_n(emu: &mut Emu, opcode: u8) {

    let register = get_arithmetic_reg_yyy(opcode);

    let byte = emu.read_register(&register);

    and(emu, byte);
}

pub fn and_u8(emu: &mut Emu) {
    let byte = emu.read_and_inc();
    and(emu, byte);
}

fn and(emu: &mut Emu, byte: u8) {
    emu.registers.accumulator = emu.registers.accumulator & byte;

    emu.registers.flags.zero = emu.registers.accumulator == 0;
    emu.registers.flags.negative = false;
    emu.registers.flags.half_carry = true;
    emu.registers.flags.carry = false;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_and_n() {
        let mut emu = Emu::default();

        and_n(&mut emu, 0xA0);
        assert_eq!(emu.cpu.m_cycles, 0);
    }

    #[test]
    fn test_and() {

        let mut emu = Emu::default();

        emu.registers.accumulator = 0x80;
        and(&mut emu, 0x7F);

        assert!(emu.registers.flags.zero);
        assert!(!emu.registers.flags.negative);
        assert!(emu.registers.flags.half_carry);
        assert!(!emu.registers.flags.carry);
    }
}