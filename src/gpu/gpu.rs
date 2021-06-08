use crate::emu::Emu;
use crate::util::Util;
use crate::flags_byte;

#[derive(Debug)]
#[derive(Default)]
pub struct GPU {
    lcd_control: LCDControl,
    lcd_status: LCDStatus,
    ly: u8,
    t_cycles: u32
}

// pub struct LCDControl {
//     lcd_display_enable: bool,
//     window_tile_map_area: bool,
//     window_enable: bool,
//     tile_data_area: bool,
//     bg_tile_map_area: bool,
//     obj_size: bool,
//     obj_enable: bool,
//     background_priority: bool
// }

// TODO move this to function / impl of "CPU" so it can access both cpu and gpu at once.

impl Emu {
    pub fn run_gpu(&mut self) {

        let t_cycles = self.cpu.m_cycles * 4;

        self.gpu.lcd_control.set_byte(self.memory.buffer[0xFF40]);
        self.gpu.lcd_status.set_byte(self.memory.buffer[0xFF41]);

        if !self.gpu.lcd_control.lcd_enable {
            return;
        }

        self.gpu.t_cycles += t_cycles as u32;

        if self.gpu.t_cycles > 114 {
            self.gpu.t_cycles -= 114;

            self.gpu.ly += 1;

            if self.gpu.ly > 153 {
                self.gpu.ly = 0;

                eprintln!("self.gpu.lcd_status = {:?}", self.gpu.lcd_status);
                eprintln!("self.gpu.lcd_control = {:?}", self.gpu.lcd_control);

                // println!("Frame");
            }
        }

        // eprintln!("self.gpu.ly = {:?}", self.gpu.ly);

        self.memory.buffer[0xFF44] = self.gpu.ly;
    }
}

flags_byte!(LCDControl,
    lcd_enable,
    window_tile_map_area,
    window_enable,
    tile_data_area,
    bg_tile_map_area,
    obj_size,
    obj_enable,
    background_priority
);

flags_byte!(LCDStatus,
    _msb,
    ly_lyc_interrupt,
    mode_2_interrupt,
    mode_1_interrupt,
    mode_0_interrupt,
    ly_equals_lyc,
    mode_flag_1,
    mode_flag_0
);
