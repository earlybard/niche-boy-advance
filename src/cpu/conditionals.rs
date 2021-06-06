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
        JumpRelativeCondition::UNCONDITIONAL => true,
        JumpRelativeCondition::Z => emu.registers.flags.zero,
        JumpRelativeCondition::NZ => !emu.registers.flags.zero,
        JumpRelativeCondition::C => emu.registers.flags.carry,
        JumpRelativeCondition::NC => !emu.registers.flags.carry
    }
}
