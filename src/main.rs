use std::borrow::Borrow;
use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;

use minifb::{Scale, Window, WindowOptions};

use emu::Emu;

use crate::gpu::gpu::GPU;
use std::process::exit;

mod cpu;
mod util;
mod gpu;
mod registers;
mod memory;
pub mod emu;
mod interrupts;

const T_CLOCK: u32 = 4194304u32;
const M_CLOCK: u32 = T_CLOCK / 4;
const FPS: u16 = 60;

#[derive(Debug)]
#[derive(Default)]
struct Emulator {
    emu: Emu,
}

/// Best docs:
/// https://gekkio.fi/files/gb-docs/gbctr.pdf
/// https://izik1.github.io/gbops/
/// https://realboyemulator.files.wordpress.com/2013/01/gbcpuman.pdf
/// http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-GPU-Timings
/// http://www.codeslinger.co.uk/pages/projects/gameboy/lcd.html
/// https://gbdev.io/pandocs/LCDC.html
impl Emulator {
    fn run(&mut self) {
        // eprintln!("rom[0] = {:x}", &self.rom[0]);
        // eprintln!("cpu = {:#?}", &self.cpu);
        self.emu.registers.pc = 0x100;
        self.emu.registers.accumulator = 0x1;
        self.emu.registers.sp = 0xFFFE;
        self.emu.registers.bc.set_word(0x0014);
        self.emu.registers.hl.set_word(0xC060);
        self.main_loop();
    }

    fn main_loop(&mut self) {

        // let mut buffer = vec![0u32; 160*144];
        //
        // let mut window = Window::new(
        //     "Test - ESC to exit",
        //     160,
        //     144,
        //     WindowOptions { scale: Scale::X4, ..WindowOptions::default() }
        // ).unwrap();
        //
        // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        loop {

            if self.emu.registers.pc == 0x1F82 {
                println!("{:?}", &self.emu.registers);
                println!("{:?}", &self.emu.registers.flags);
            }
            //
            println!("PC: {:#6X?}", self.emu.registers.pc);
            let opcode = self.emu.read_and_inc();
            println!("OP: {:#04X?}", opcode);
            println!("{:?}", &self.emu.registers);
            println!("{:?}", &self.emu.registers.flags);

            let m_cycles = self.emu.run_operand(opcode);

            if m_cycles == 0 {
                // Unknown.
                println!("Unknown opcode: {:#4X?}", opcode);
                println!("{:?}", &self.emu.registers);
                println!("{:?}", &self.emu.registers.flags);
                break;
            }

            self.emu.run_gpu(m_cycles * 4);

            // if window.is_open() {
            //    window.update_with_buffer(&buffer, 160, 144) .unwrap();
            // }
            // println!("{}", cycles);

            // println!("{:?}", &self.cpu.registers);
            // println!("{:?}", &self.cpu.registers.flags);
        }
        // eprintln!("opcode = {:#?}", opcode);
    }
}


fn main() {

    // let rom = File::open("C:\\Users\\Dylan\\Downloads\\Pokemon - Red Version (UE)[!]\\Pokemon Red.gb");

    // Maximum size of GB ROM: http://www.codeslinger.co.uk/pages/projects/gameboy/beginning.html
    // let mut rom = [0u8; 200000];
    let mut boot = File::open("roms/Pokemon Red.gb").unwrap();
    // let mut boot = File::open("src/dmg_boot.bin").unwrap();

    let mut emu = Emulator::default();
    boot.read(&mut emu.emu.memory.buffer).unwrap();

    emu.run();


    // let he = hex::encode(rom);
    // println!("{}", he);

    // println!("{:?}", rom);

    // println!("Hello, world!");
}
