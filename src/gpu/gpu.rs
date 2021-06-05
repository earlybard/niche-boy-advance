use crate::cpu::emu::Emu;

#[derive(Debug)]
#[derive(Default)]
pub struct GPU {
    ly: u8,
    t_cycles: u32
}

// TODO move this to function / impl of "CPU" so it can access both cpu and gpu at once.

impl Emu {
    pub fn run_gpu(&mut self, t_cycles: u8) {

        self.gpu.t_cycles += t_cycles as u32;

        if self.gpu.t_cycles > 114 {
            self.gpu.t_cycles -= 114;

            self.gpu.ly += 1;

            if self.gpu.ly > 153 {
                self.gpu.ly = 0;
                println!("Frame");
            }
        }

        eprintln!("self.gpu.ly = {:?}", self.gpu.ly);

        self.memory.buffer[0xFF44] = self.gpu.ly;
    }
}