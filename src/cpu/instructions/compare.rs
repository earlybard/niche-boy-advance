use crate::cpu::emu::Emu;

pub fn compare(cpu: &mut Emu) -> u8 {
    let constant = cpu.read_and_inc();

    let (result, carry) = cpu.registers.accumulator.overflowing_sub(constant);

    // Bitwise AND with 00001111 to only get the lower nibble for half_carry.
    let (_, half_carry) = (cpu.registers.accumulator & 0xF).overflowing_sub(constant & 0xF);

    cpu.registers.flags.zero = result == 0;
    cpu.registers.flags.subtraction = true;
    cpu.registers.flags.carry = carry;
    cpu.registers.flags.half_carry = half_carry;

    2
}
