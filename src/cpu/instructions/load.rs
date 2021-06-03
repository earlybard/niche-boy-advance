use crate::cpu::cpu::{CPU, Reg};
use crate::util::Util;
use crate::cpu::register::Register;

pub enum LoadMode {
    WORD,
    FF00
}

pub fn load(cpu: &mut CPU, mode: LoadMode) -> u8 {
    // LD (nn), A

    let addr = match mode {
        LoadMode::WORD => cpu.read_u16_and_inc(),
        LoadMode::FF00 => 0xFF00 + cpu.read_and_inc() as u16
    };

    cpu.memory.write_byte(addr, cpu.registers.accumulator);
    3
}

pub fn load_to_register(cpu: &mut CPU, register: Register) -> u8 {

    let value = cpu.read_and_inc();
    cpu.registers.set_value(&register, value);
    2
}

pub fn load_control_to_register(cpu: &mut CPU, register: Register) -> u8 {

    let index = cpu.read_and_inc();
    let control = cpu.memory.read_byte(Util::bytes_to_word(0xFF, index));

    cpu.registers.set_value(&register, control);

    3
}

pub fn load_register_to_register(cpu: &mut CPU, to: Register, from: Register) -> u8 {

    cpu.registers.set_value(&from, cpu.registers.get_value(&to));

    1
}