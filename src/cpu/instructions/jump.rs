use crate::emu::Emu;
use crate::cpu::conditionals::{Condition, check_condition};

pub fn jump_relative(emu: &mut Emu, condition: Condition) {

    let offset = emu.read_and_inc() as i8;

    if !check_condition(emu, condition) {
        return;
    }

    if offset.is_negative() {
        // TODO wrapping sub?
        emu.registers.program_counter -= offset.abs() as u16;
    } else {
        emu.registers.program_counter += offset.abs() as u16;
    }

    emu.cpu.cycle();
}

pub fn jump_nn(cpu: &mut Emu) {
    // JP nn

    cpu.registers.program_counter = cpu.read_u16_and_inc();
}
