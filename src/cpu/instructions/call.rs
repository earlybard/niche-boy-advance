use crate::cpu::cpu::CPU;

pub fn call(cpu: &mut CPU) -> u8 {
    let value = cpu.read_u16_and_inc();

    cpu.push_to_stack(cpu.registers.pc);
    cpu.registers.pc = value;

    3
}