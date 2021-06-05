use crate::cpu::cpu::{Emu, Reg};
use crate::util::Util;
use crate::cpu::register::Register;
use crate::cpu::register::Register::{B, C, D, E, H, L, HLPOINTER, A};

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

pub fn load_to_register(cpu: &mut Emu, register: Register) -> u8 {

    let value = cpu.read_and_inc();
    cpu.registers.set_value(&register, value);
    2
}

pub fn load_control_to_register(cpu: &mut Emu, register: Register) -> u8 {

    let index = cpu.read_and_inc();
    let control = cpu.memory.read_byte(Util::bytes_to_word(0xFF, index));

    cpu.registers.set_value(&register, control);

    3
}

fn get_arithmetic_register(code: u8) -> Register {
    return match code {
        0 => B,
        1 => C,
        2 => D,
        3 => E,
        4 => H,
        5 => L,
        6 => HLPOINTER,
        7 => A,
        _ => panic!("Not a valid arithemtic register" + code.to_string())
    }
}

pub fn load_rr(cpu: &mut Emu, opcode: u8) -> u8 {

    let xxx = (opcode & 0b00111000) >> 3;
    let yyy = opcode & 0b00000111;

    let x = get_arithmetic_register(xxx);
    let y = get_arithmetic_register(yyy);

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_rr() {

        let mut cpu = Emu::default();
        load_rr(&mut cpu, 0x53);
    }
}

pub fn load_register_to_register(cpu: &mut Emu, from: Register, to: Register) -> u8 {

    cpu.registers.set_value(&from, cpu.registers.get_value(&to));

    1
}