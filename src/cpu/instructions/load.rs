use crate::emu::{Emu};
use crate::util::Util;
use crate::registers::register::{Register, RegisterPair, get_arithmetic_reg_xxx, get_arithmetic_reg_yyy};
use crate::registers::register::Register::{HLPOINTER};

pub enum LoadMode {
    WORD,
    FF00
}

pub fn load(cpu: &mut Emu, mode: LoadMode) -> u8 {
    // LD (nn), A

    let addr = match mode {
        LoadMode::WORD => cpu.read_u16_and_inc(),
        LoadMode::FF00 => 0xFF00 + cpu.read_and_inc() as u16
    };

    cpu.memory.write_byte(addr, cpu.registers.accumulator);
    3
}

pub fn load_control_to_register(cpu: &mut Emu, register: Register) -> u8 {

    let index = cpu.read_and_inc();
    let control = cpu.memory.read_byte(Util::bytes_to_word(0xFF, index));

    cpu.set_reg(&register, control);

    3
}

pub fn load_rr_nn(emu: &mut Emu, register: RegisterPair) -> u8 {

    let word = emu.read_u16_and_inc();
    emu.set_reg_pair(&register, word);

    3
}

pub fn load_r_n(cpu: &mut Emu, opcode: u8) -> u8 {

    let to = get_arithmetic_reg_xxx(opcode);

    let value = cpu.read_and_inc();
    cpu.set_reg(&to, value);
    2
}

pub fn load_rr(cpu: &mut Emu, opcode: u8) -> u8 {

    let to = get_arithmetic_reg_xxx(opcode);
    let from = get_arithmetic_reg_yyy(opcode);

    load_register_to_register(cpu, to, from)
}

fn load_register_to_register(emu: &mut Emu, to: Register, from: Register) -> u8 {

    let mut cycles = 1u8;

    if matches!(from, HLPOINTER) || matches!(to, HLPOINTER) {
        cycles = 2;
    }

    if matches!(from, HLPOINTER) &&  matches!(to, HLPOINTER) {
        emu.cpu.halted = true;
        cycles = 1
    } else {
        emu.set_reg(&to, emu.get_reg(&from));
    }

    cycles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_rr() {

        let mut emu = Emu::default();
        emu.registers.de.first = 0x10;
        emu.registers.de.second = 0x20;

        // LD D, E
        load_register_to_register(&mut emu, D, E);

        assert_eq!(emu.registers.de.first, 0x20);
        assert_eq!(emu.registers.de.second, 0x20);

        emu.registers.de.first = 0x10;
        emu.registers.de.second = 0x20;

        // Test from opcode.
        load_rr(&mut emu, 0x53);

        assert_eq!(emu.registers.de.first, 0x20);
        assert_eq!(emu.registers.de.second, 0x20);
    }

    #[test]
    fn test_load_rr_hlpointer() {
        let mut emu = Emu::default();

        emu.registers.de.first = 20;
        emu.registers.hl.set_word(0xFFAA);
        emu.memory.buffer[0xFFAA] = 10;

        let cycles = load_register_to_register(&mut emu, HLPOINTER, D);

        assert_eq!(emu.memory.buffer[0xFFAA], 20);
        assert_eq!(cycles, 2);
    }
}
