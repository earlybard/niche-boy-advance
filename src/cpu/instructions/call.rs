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

pub fn ret(emu: &mut Emu) {
    emu.registers.program_counter = emu.pop_from_stack();
    emu.cpu.cycle();
}
