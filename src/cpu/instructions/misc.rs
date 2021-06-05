use crate::cpu::cpu::Emu;

pub fn noop() -> u8 {
    1
}

pub fn di(cpu: &mut Emu) -> u8 {
    cpu.interrupts.ime = false;
    1
}
