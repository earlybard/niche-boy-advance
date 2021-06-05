use crate::cpu::emu::Emu;
use crate::cpu::register::Register;

pub fn xor(cpu: &mut Emu, register: Register) -> u8 {

    cpu.registers.accumulator ^= cpu.get_reg(&register);

    cpu.registers.flags.reset();

    if cpu.registers.accumulator == 0
    {
        cpu.registers.flags.zero = true
    }

    1
}
