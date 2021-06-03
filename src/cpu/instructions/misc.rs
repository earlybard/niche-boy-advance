use crate::cpu::cpu::CPU;

pub fn noop() -> u8 {
    1
}

pub fn di(cpu: &mut CPU) -> u8 {
    cpu.interrupts.ime = false;
    1
}
