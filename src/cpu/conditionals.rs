use crate::emu::Emu;

pub enum Condition {
    UNCONDITIONAL,
    Z,
    NZ,
    C,
    NC
}

pub fn check_condition(emu: &Emu, condition: Condition) -> bool {
    match condition {
        Condition::UNCONDITIONAL => true,
        Condition::Z => emu.registers.flags.zero,
        Condition::NZ => !emu.registers.flags.zero,
        Condition::C => emu.registers.flags.carry,
        Condition::NC => !emu.registers.flags.carry
    }
}
