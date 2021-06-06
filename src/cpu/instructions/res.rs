use crate::emu::Emu;
use crate::registers::register::RegisterType;
use crate::util::Util;

pub fn res(cpu: &mut Emu, bit: u8, register: RegisterType) {

    let val = cpu.read_register(&register);

    let result = Util::reset_flag(val, bit);

    cpu.write_register(&register, result);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_res() {

        let mut cpu = Emu::default();

        cpu.registers.hl.first = 0b10111001;
        res(&mut cpu, 7, RegisterType::H);
        assert_eq!(cpu.registers.hl.first, 0b00111001);

        res(&mut cpu, 4, RegisterType::H);
        assert_eq!(cpu.registers.hl.first, 0b00101001);
    }
}
