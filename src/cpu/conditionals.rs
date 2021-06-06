use crate::emu::Emu;

pub enum Condition {
    Unconditional,
    Zero,
    NotZero,
    Carry,
    NotCarry
}

pub fn check_condition(emu: &Emu, condition: Condition) -> bool {
    match condition {
        Condition::Unconditional => true,
        Condition::Zero => emu.registers.flags.zero,
        Condition::NotZero => !emu.registers.flags.zero,
        Condition::Carry => emu.registers.flags.carry,
        Condition::NotCarry => !emu.registers.flags.carry
    }
}
