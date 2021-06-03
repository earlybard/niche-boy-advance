use crate::cpu::cpu::CPU;

#[derive(Debug)]
#[derive(Default)]
pub struct GPU {
    ly: u8,
    t_cycles: u32
}

// TODO move this to function / impl of "CPU" so it can access both cpu and gpu at once.

impl GPU {
    pub fn go(&mut self, &mut cpu: CPU, t_cycles: u8) {

        self.t_cycles += t_cycles as u32;

        if self.t_cycles > 114 {
            self.t_cycles -= 114;

            self.ly += 1;

            if self.ly > 153 {
                self.ly = 0;
                println!("Frame");
            }
        }

        cpu.memory.buffer[0xFF44] = self.ly;
    }
}