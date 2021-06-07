use crate::emu::Emu;
use crate::util::Util;

#[derive(Debug)]
#[derive(Default)]
pub struct GPU {
    ly: u8,
    t_cycles: u32
}

// TODO move this to function / impl of "CPU" so it can access both cpu and gpu at once.

impl Emu {
    pub fn run_gpu(&mut self) {

        let t_cycles = self.cpu.m_cycles * 4;

        let lcd_control = self.memory.buffer[0xFF40];
        // let lcd_control = self.read_byte_from_memory(0xFF40);
        let lcd_enable = Util::get_flag(lcd_control, 7);

        if !lcd_enable {
            return;
        }

        self.gpu.t_cycles += t_cycles as u32;

        if self.gpu.t_cycles > 114 {
            self.gpu.t_cycles -= 114;

            self.gpu.ly += 1;

            if self.gpu.ly > 153 {
                self.gpu.ly = 0;
                // println!("Frame");
            }
        }

        // eprintln!("self.gpu.ly = {:?}", self.gpu.ly);

        self.memory.buffer[0xFF44] = self.gpu.ly;
    }
}