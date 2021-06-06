use crate::emu::Emu;

pub fn noop() {

}

pub fn di(cpu: &mut Emu) {
    cpu.interrupts.ime = false;
}
