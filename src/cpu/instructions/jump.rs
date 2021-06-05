use crate::cpu::cpu::Emu;

pub enum JumpRelativeCondition {
    UNCONDITIONAL,
    Z,
    NZ,
    C,
    NC
}

pub fn jump_relative(cpu: &mut Emu, condition: JumpRelativeCondition) -> u8 {
    // JR Z, i8
    // 2M without branch, 3M with branch

    let should_jump = match condition {
        JumpRelativeCondition::UNCONDITIONAL => true,
        JumpRelativeCondition::Z => cpu.registers.flags.zero,
        JumpRelativeCondition::NZ => !cpu.registers.flags.zero,
        JumpRelativeCondition::C => true,
        JumpRelativeCondition::NC => true
    };

    let offset = cpu.read_and_inc() as i8;

    return if should_jump {
        if offset.is_negative() {
            cpu.registers.pc -= offset.abs() as u16;
        } else {
            cpu.registers.pc += offset.abs() as u16;
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
