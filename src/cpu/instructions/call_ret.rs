use crate::emu::Emu;
use crate::cpu::conditionals::{Condition, check_condition};

pub fn call(emu: &mut Emu, condition: Condition) {

    // FIXME Even though this gets read every time on a real gb,
    // we can optimize by just incrementing pc and cycles if the condition isn't true.
    let value = emu.read_u16_and_inc();

    if check_condition(emu, condition) {
        emu.cpu.cycle();
        emu.push_to_stack(emu.registers.program_counter);
        emu.registers.program_counter = value;
    }
}

pub fn restart(emu: &mut Emu, address: u8) {
    emu.cpu.cycle();
    emu.push_to_stack(emu.registers.program_counter);
    emu.registers.program_counter = address as u16;
}

pub fn ret(emu: &mut Emu, condition: Condition) {

    // The conditional versions all cycle an extra time.
    if !matches!(condition, Condition::Unconditional) {
        emu.cpu.cycle();
    }

    if check_condition(emu, condition) {
        emu.registers.program_counter = emu.pop_from_stack();
        emu.cpu.cycle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ret() {

        let mut emu = Emu::default();

        ret(&mut emu, Condition::Unconditional);
        assert_eq!(emu.cpu.m_cycles, 3);

        emu.cpu.m_cycles = 0;

        ret(&mut emu, Condition::NotZero);
        assert_eq!(emu.cpu.m_cycles, 4);

        emu.cpu.m_cycles = 0;

        ret(&mut emu, Condition::Zero);
        assert_eq!(emu.cpu.m_cycles, 1);
    }
}
