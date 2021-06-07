use crate::emu::Emu;
use crate::registers::register::RegisterType;

pub fn xor(cpu: &mut Emu, register: RegisterType) {

    cpu.registers.accumulator ^= cpu.read_register(&register);
    cpu.registers.flags.reset();
    cpu.registers.flags.zero = cpu.registers.accumulator == 0;
}
