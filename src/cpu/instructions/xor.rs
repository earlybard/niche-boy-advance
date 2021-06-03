use crate::cpu::cpu::CPU;
use crate::cpu::register::Register;

pub fn xor(cpu: &mut CPU, register: Register) -> u8 {

    cpu.registers.accumulator ^= cpu.registers.get_value(&register);

    cpu.registers.flags.reset();

    if cpu.registers.accumulator == 0
    {
        cpu.registers.flags.zero = true
    }

    1
}
