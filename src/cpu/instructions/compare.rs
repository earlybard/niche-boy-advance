use crate::emu::Emu;
use crate::util::Util;
use crate::registers::register::RegisterType;

pub fn compare(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);
    compare_internal(emu, byte);
}

fn compare_internal(emu: &mut Emu, byte: u8) {

    let (value, hc, c) = Util::sub_with_flags(emu.registers.accumulator, byte);

    emu.registers.flags.zero = value == 0;
    emu.registers.flags.negative = true;
    emu.registers.flags.carry = c;
    emu.registers.flags.half_carry = hc;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compare() {

        let mut emu = Emu::default();

        for n in 0x01..=0x0F {
            emu.registers.accumulator = n;
            compare_internal(&mut emu, 0x91);

            assert!(!emu.registers.flags.zero);
            assert!(emu.registers.flags.negative);
            assert!(!emu.registers.flags.half_carry);
            assert!(emu.registers.flags.carry);
        }

        emu.registers.accumulator = 0x10;
        compare_internal(&mut emu, 0x91);

        assert!(!emu.registers.flags.zero);
        assert!(emu.registers.flags.negative);
        assert!(emu.registers.flags.half_carry);
        assert!(emu.registers.flags.carry);

        emu.registers.accumulator = 0x91;
        compare_internal(&mut emu, 0x91);

        assert!(emu.registers.flags.zero);
        assert!(emu.registers.flags.negative);
        assert!(!emu.registers.flags.half_carry);
        assert!(!emu.registers.flags.carry);

        eprintln!("emu.registers.flags = {:?}", emu.registers.flags);
    }
}