use crate::emu::Emu;
use crate::cpu::conditionals::{Condition, check_condition};

pub fn jump_relative(emu: &mut Emu, condition: Condition) -> u8 {
    // JR Z, i8
    // 2M without branch, 3M with branch

    let should_jump = check_condition(emu, condition);

    let offset = emu.read_and_inc() as i8;

    return if should_jump {
        if offset.is_negative() {
            emu.registers.pc -= offset.abs() as u16;
        } else {
            emu.registers.pc += offset.abs() as u16;
        }
        3
    } else {
        2
    }
}

pub fn jump(cpu: &mut Emu) -> u8 {
    // JP nn

    cpu.registers.pc = cpu.read_u16_and_inc();
    3
}
