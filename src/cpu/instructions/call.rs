use crate::emu::Emu;
use crate::cpu::conditionals::{Condition, check_condition};

pub fn call(emu: &mut Emu, condition: Condition) -> u8 {

    // FIXME Even though this gets read every time on a real gb,
    // we can optimize by just incrementing pc if the condition isn't true.
    let value = emu.read_u16_and_inc();

    if check_condition(emu, condition) {
        emu.push_to_stack(emu.registers.pc);
        emu.registers.pc = value;
        return 6;
    }

    3
}

pub fn ret(emu: &mut Emu) -> u8 {
    emu.registers.pc = emu.pop_from_stack();
    4
}