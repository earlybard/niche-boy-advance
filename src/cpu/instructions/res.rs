use crate::cpu::emu::Emu;
use crate::cpu::register::Register;

pub fn res(cpu: &mut Emu, bit: u8, register: Register) -> u8 {

    let val = cpu.get_reg(&register);

    // e.g. for bit=3 11110111, bit=2 11111011
    let reset = !(1u8 << bit);

    cpu.set_reg(&register, val & reset);

    2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_res() {

        let mut cpu = Emu::default();

        cpu.registers.hl.first = 0b10111001;
        res(&mut cpu, 7, Register::H);
        assert_eq!(cpu.registers.hl.first, 0b00111001);

        res(&mut cpu, 4, Register::H);
        assert_eq!(cpu.registers.hl.first, 0b00101001);
    }
}