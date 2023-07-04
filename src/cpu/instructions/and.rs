use crate::emu::Emu;
use crate::registers::register::{RegisterType};

pub fn and(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);
    and_internal(emu, byte);
}

fn and_internal(emu: &mut Emu, byte: u8) {
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

        and_internal(&mut emu, 0xA0);
        assert_eq!(emu.cpu.m_cycles, 0);
    }

    #[test]
    fn test_and() {

        let mut emu = Emu::default();

        emu.registers.accumulator = 0x80;
        and_internal(&mut emu, 0x7F);

        assert!(emu.registers.flags.zero);
        assert!(!emu.registers.flags.negative);
        assert!(emu.registers.flags.half_carry);
        assert!(!emu.registers.flags.carry);
    }
}