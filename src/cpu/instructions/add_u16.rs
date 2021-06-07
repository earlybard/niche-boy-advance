use crate::emu::Emu;
use crate::registers::register::RegisterPairType;
use crate::util::Util;

/// Add the register r to the HL register.
pub fn add_hl_rr(emu: &mut Emu, register_pair: RegisterPairType) {

    let left = emu.read_register_pair(&RegisterPairType::HL);
    let right = emu.read_register_pair(&register_pair);
    let (result, half_carry, carry) = Util::add_u16_with_flags(left, right);

    emu.write_register_pair(&RegisterPairType::HL, result);

    emu.registers.flags.negative = false;
    emu.registers.flags.half_carry = half_carry;
    emu.registers.flags.carry = carry;

    emu.cpu.cycle();
}
