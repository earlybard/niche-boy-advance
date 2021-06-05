use crate::emu::Emu;

pub fn call(cpu: &mut Emu) -> u8 {
    let value = cpu.read_u16_and_inc();

    cpu.push_to_stack(cpu.registers.pc);
    cpu.registers.pc = value;

    3
}

pub fn ret(emu: &mut Emu) -> u8 {
    emu.registers.pc = emu.pop_from_stack();
    4
}