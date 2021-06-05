use crate::emu::Emu;

pub fn compare(emu: &mut Emu) -> u8 {
    let byte = emu.read_and_inc();
    compare_with(emu, byte)
}

fn compare_with(cpu: &mut Emu, byte: u8) -> u8 {
    let (result, carry) = cpu.registers.accumulator.overflowing_sub(byte);

    // Bitwise AND with 00001111 to only get the lower nibble for half_carry.
    let (_, half_carry) = (cpu.registers.accumulator & 0xF).overflowing_sub(byte & 0xF);

    cpu.registers.flags.zero = result == 0;
    cpu.registers.flags.negative = true;
    cpu.registers.flags.carry = carry;
    cpu.registers.flags.half_carry = half_carry;

    2
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