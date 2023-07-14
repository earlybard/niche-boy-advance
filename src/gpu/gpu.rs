use crate::emu::Emu;
use crate::gpu::framebuffer::FrameBuffer;
use crate::memory::addresses::{LY, LYLC};
use crate::memory::bitsets::{LCDControl, LCDStatus};

#[derive(Debug)]
#[derive(Default)]
pub struct GPU {
    lx: u16,
    fb: FrameBuffer
}

#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
pub enum GpuMode {
    #[default]
    OAM,
    PixelTransfer,
    HBlank,
    VBlank
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

    pub fn cycle_gpu(&mut self) {

        if !LCDControl::get::lcd_enable(&self.memory) {
            return;
        }

        // Scanlines
        if self.get_gpu_mode() != GpuMode::VBlank {
            // Not blanking, do scanline things

            if self.gpu.lx == 0 { self.set_gpu_mode(GpuMode::OAM); }

            if self.gpu.lx == 80 { self.set_gpu_mode(GpuMode::PixelTransfer); }

            // TODO hblank isn't exactly 172 after pixel transfer, it depends
            if self.gpu.lx == 80 + 172 { self.set_gpu_mode(GpuMode::HBlank); }
        }

        self.gpu.lx += 4;

        if self.gpu.lx == 456 {

            // Next scanline
            self.memory[LY] += 1;
            // self.memory.inc(LY);
            // self.memory.buffer[LY as usize] += 1;
            self.lyc_check();
            self.gpu.lx = 0;

            if self.memory[LY] == 144 {
                // Render frame
                // self.gpu.fb.render_full_tilemap(&self.memory);
                self.gpu.fb.update();
                self.set_gpu_mode(GpuMode::VBlank);
            }

            if self.memory[LY] == 154 {
                self.memory[LY] = 0;
                self.set_gpu_mode(GpuMode::OAM);
            }
        }
    }

    pub fn lyc_check(&mut self) {
        let coincidence = self.memory[LYLC] == self.memory[LY];
        LCDStatus::put::ly_equals_lyc(&mut self.memory, coincidence);
    }

    // pub fn run_gpu(&mut self) {
    //
    //     let t_cycles = self.cpu.m_cycles * 4;
    //     println!("{}", t_cycles);
    //
    //     self.gpu.lcd_control.set_byte(self.memory.buffer[0xFF40]);
    //     self.gpu.lcd_status.set_byte(self.memory.buffer[0xFF41]);
    //
    //     if !self.gpu.lcd_control.lcd_enable {
    //         return;
    //     }
    //
    //     self.gpu.lx += t_cycles as u32;
    //
    //     if self.gpu.lx > 114 {
    //         self.gpu.lx -= 114;
    //
    //         self.gpu.ly += 1;
    //
    //         if self.gpu.ly > 153 {
    //             self.gpu.ly = 0;
    //
    //             eprintln!("self.gpu.lcd_status = {:?}", self.gpu.lcd_status);
    //             eprintln!("self.gpu.lcd_control = {:?}", self.gpu.lcd_control);
    //
    //             // println!("Frame");
    //         }
    //     }
    //
    //     // eprintln!("self.gpu.ly = {:?}", self.gpu.ly);
    //
    //     self.memory.buffer[0xFF44] = self.gpu.ly;
    // }
}

impl Emu {
    pub fn get_gpu_mode(&mut self) -> GpuMode {
        let flag_0 = LCDStatus::get::mode_flag_0(&mut self.memory);
        let flag_1 = LCDStatus::get::mode_flag_1(&mut self.memory);

        if      flag_0 && flag_1    { GpuMode::PixelTransfer }
        else if flag_0              { GpuMode::VBlank }
        else if flag_1              { GpuMode::OAM }
        else                        { GpuMode::HBlank }
    }

    pub fn set_gpu_mode(&mut self, mode: GpuMode) {
        match mode {
            GpuMode::OAM => {
                LCDStatus::res::mode_flag_0(&mut self.memory);
                LCDStatus::set::mode_flag_1(&mut self.memory);
            }
            GpuMode::PixelTransfer => {
                LCDStatus::set::mode_flag_0(&mut self.memory);
                LCDStatus::set::mode_flag_1(&mut self.memory);
            }
            GpuMode::HBlank => {
                LCDStatus::res::mode_flag_0(&mut self.memory);
                LCDStatus::res::mode_flag_1(&mut self.memory);
            }
            GpuMode::VBlank => {
                LCDStatus::set::mode_flag_0(&mut self.memory);
                LCDStatus::res::mode_flag_1(&mut self.memory);
            }
        }
    }
}

impl GPU {
    pub fn enable_window(&mut self) {
        self.fb.enable_window();
    }
}

