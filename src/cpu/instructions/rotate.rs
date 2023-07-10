use crate::emu::Emu;
use crate::registers::register::RegisterType;
use crate::registers::register::RegisterType::A;
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
    emu.registers.flags.zero = output == 0;

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
    emu.registers.flags.zero = output == 0;

    output
}

fn rotate_right_through_carry(emu: &mut Emu, byte: u8) -> u8 {
    let rotated = Util::get_bit(byte, 0);
    let carry = emu.registers.flags.carry;

    let mut output = byte >> 1;

    if carry {
        output = Util::set_bit(output, 7)
    }

    emu.registers.flags.carry = rotated;
    emu.registers.flags.half_carry = false;
    emu.registers.flags.negative = false;
    emu.registers.flags.zero = output == 0;

    output
}


fn rotate_left_through_carry(emu: &mut Emu, byte: u8) -> u8 {
    let rotated = Util::get_bit(byte, 7);
    let carry = emu.registers.flags.carry;

    let mut output = byte << 1;

    if carry {
        output = Util::set_bit(output, 0)
    }

    emu.registers.flags.carry = rotated;
    emu.registers.flags.half_carry = false;
    emu.registers.flags.negative = false;
    emu.registers.flags.zero = output == 0;

    output
}

pub fn rlca(emu: &mut Emu) {
    rlc(emu, A);
    emu.registers.flags.zero = true;
}

pub fn rla(emu: &mut Emu) {
    rl(emu, A);
    emu.registers.flags.zero = true;
}

pub fn rrca(emu: &mut Emu) {
    rrc(emu, A);
    emu.registers.flags.zero = true;
}

pub fn rra(emu: &mut Emu) {
    rr(emu, A);
    emu.registers.flags.zero = true;
}

/// Rotate left circular accumulator.
/// Rotate guide: http://www.chebucto.ns.ca/~af380/z-80-g.htm
pub fn rlc(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);
    let value = rotate_left(emu, byte);
    emu.write_register(&register, value);
}

pub fn rl(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);
    let value = rotate_left_through_carry(emu, byte);
    emu.write_register(&register, value);
}

/// Rotate right circular accumulator.
pub fn rrc(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);
    let value = rotate_right(emu, byte);
    emu.write_register(&register, value);
}

pub fn rr(emu: &mut Emu, register: RegisterType) {
    let byte = emu.read_register(&register);
    let value = rotate_right_through_carry(emu, byte);
    emu.write_register(&register, value);
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rl() {
        let mut emu = Emu::default();

        let result = rotate_left(&mut emu, 0b11110000);
        println!("{:#b}", result);
        assert_eq!(0b11100001, result);
    }

    #[test]
    fn test_rl_carry() {
        let mut emu = Emu::default();
        emu.registers.flags.carry = true;

        let result = rotate_left_through_carry(&mut emu, 0b00001111);
        println!("{:#010b}", result);
        assert_eq!(0b00011111, result);
        assert_eq!(false, emu.registers.flags.carry);

        let result = rotate_left_through_carry(&mut emu, 0b00001111);
        println!("{:#010b}", result);
        assert_eq!(0b00011110, result);
        assert_eq!(false, emu.registers.flags.carry);
    }

    #[test]
    fn test_rr() {
        let mut emu = Emu::default();

        let result = rotate_right(&mut emu, 0b11110000);
        println!("{:#b}", result);
        assert_eq!(0b01111000, result);
    }

    #[test]
    fn test_rr_carry() {
        let mut emu = Emu::default();
        emu.registers.flags.carry = true;

        let result = rotate_right_through_carry(&mut emu, 0b00001111);
        println!("{:#010b}", result);
        assert_eq!(0b10000111, result);
        assert_eq!(true, emu.registers.flags.carry);

        emu.registers.flags.carry = false;
        let result = rotate_right_through_carry(&mut emu, 0b00001111);
        println!("{:#010b}", result);
        assert_eq!(0b00000111, result);
        assert_eq!(true, emu.registers.flags.carry);
    }
}
