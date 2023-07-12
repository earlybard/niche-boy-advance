use crate::emu::Emu;

pub fn noop() {

}

pub fn di(cpu: &mut Emu) {
    println!("di");
    cpu.interrupts.ime = false;
}

pub fn cpl(emu: &mut Emu) {
    let a = emu.registers.accumulator;
    emu.registers.accumulator = !a;
    emu.registers.flags.negative = true;
    emu.registers.flags.half_carry = true;
}

pub fn ccf(emu: &mut Emu) {
    emu.registers.flags.negative = false;
    emu.registers.flags.half_carry = false;
    emu.registers.flags.carry = !emu.registers.flags.carry;
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cpl() {
        let mut emu = Emu::default();
        emu.registers.accumulator = 0b11001100;

        cpl(&mut emu);

        assert_eq!(0b00110011, emu.registers.accumulator);
    }
}