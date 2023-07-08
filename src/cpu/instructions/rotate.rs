use crate::emu::Emu;
use crate::util::Util;

fn rotate_left(emu: &mut Emu, byte: u8) -> u8 {
    let rotated = Util::get_bit(byte, 7);
    let mut output = byte << 1;

    if rotated {
        output = Util::set_bit(output, 0)
    }

    emu.registers.flags.carry = rotated;
    emu.registers.flags.half_carry = false;
    emu.registers.flags.negative = false;
    emu.registers.flags.zero = false;

    output
}

fn rotate_right(emu: &mut Emu, byte: u8) -> u8 {
    let rotated = Util::get_bit(byte, 0);

    let mut output = byte >> 1;
    if rotated {
        output = Util::set_bit(output, 7)
    }

    emu.registers.flags.carry = rotated;
    emu.registers.flags.half_carry = false;
    emu.registers.flags.negative = false;
    emu.registers.flags.zero = false;

    output
}


/// Rotate left circular accumulator.
/// Rotate guide: http://www.chebucto.ns.ca/~af380/z-80-g.htm
pub fn rlca(emu: &mut Emu) {
    emu.registers.accumulator = rotate_left(emu, emu.registers.accumulator);
}

// TODO RLA

/// Rotate right circular accumulator.
pub fn rrca(emu: &mut Emu) {
    emu.registers.accumulator = rotate_right(emu, emu.registers.accumulator);
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rleft() {
        let mut emu = Emu::default();

        let result = rotate_left(&mut emu, 0b11110000);
        println!("{:#b}", result);
        assert_eq!(0b11100001, result);
    }

    #[test]
    fn test_rright() {
        let mut emu = Emu::default();

        let result = rotate_right(&mut emu, 0b11110000);
        println!("{:#b}", result);
        assert_eq!(0b01111000, result);
    }
}
