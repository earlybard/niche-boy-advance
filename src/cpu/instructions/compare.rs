use crate::emu::Emu;
use crate::util::Util;

pub fn compare(emu: &mut Emu) {
    let byte = emu.read_and_inc();
    compare_with(emu, byte);
}

fn compare_with(cpu: &mut Emu, byte: u8) {

    let (result, half_carry, carry) = Util::sub_with_carry_flags(cpu.registers.accumulator, byte);

    cpu.registers.flags.zero = result == 0;
    cpu.registers.flags.negative = true;
    cpu.registers.flags.carry = carry;
    cpu.registers.flags.half_carry = half_carry;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_compare() {

        let mut emu = Emu::default();

        for n in 0x01..=0x0F {
            emu.registers.accumulator = n;
            compare_with(&mut emu, 0x91);

            assert!(!emu.registers.flags.zero);
            assert!(emu.registers.flags.negative);
            assert!(!emu.registers.flags.half_carry);
            assert!(emu.registers.flags.carry);
        }

        emu.registers.accumulator = 0x10;
        compare_with(&mut emu, 0x91);

        assert!(!emu.registers.flags.zero);
        assert!(emu.registers.flags.negative);
        assert!(emu.registers.flags.half_carry);
        assert!(emu.registers.flags.carry);

        emu.registers.accumulator = 0x91;
        compare_with(&mut emu, 0x91);

        assert!(emu.registers.flags.zero);
        assert!(emu.registers.flags.negative);
        assert!(!emu.registers.flags.half_carry);
        assert!(!emu.registers.flags.carry);

        eprintln!("emu.registers.flags = {:?}", emu.registers.flags);
    }
}