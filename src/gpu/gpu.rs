use minifb::{Scale, Window, WindowOptions};
use crate::emu::Emu;
use crate::flags_byte;
use crate::gpu::framebuffer::FrameBuffer;
use crate::memory::memory::Memory;

#[derive(Debug)]
#[derive(Default)]
pub struct GPU {
    pub(crate) lcd_control: LCDControl,
    pub(crate) lcd_status: LCDStatus,
    pub ly: u8,
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

        // if !self.gpu.lcd_control.lcd_enable {
        //     return;
        // }

        // Check LYC TODO could do this whenever ly or lyc change?
        if self.gpu.ly == self.memory.buffer[0xFF45] {
            self.gpu.lcd_status.ly_equals_lyc = true;

            if self.gpu.lcd_status.ly_lyc_interrupt {
                // Interrupt here
                println!("ly=lyc interrupt");
            }
        } else {
            self.gpu.lcd_status.ly_equals_lyc = false;
        }

        // Scanlines

        if self.gpu.get_mode() != GpuMode::VBlank {
            // Not blanking, do scanline things

            if self.gpu.lx == 0 { self.gpu.set_mode(GpuMode::OAM); }

            if self.gpu.lx == 80 { self.gpu.set_mode(GpuMode::PixelTransfer); }

            // TODO hblank isn't exactly 172 after pixel transfer, it depends
            if self.gpu.lx == 80 + 172 { self.gpu.set_mode(GpuMode::HBlank); }
        }

        self.gpu.lx += 4;

        if self.gpu.lx == 456 {

            // Next scanline
            self.gpu.ly += 1;
            self.gpu.lx = 0;

            if self.gpu.ly == 144 {
                // Render frame
                self.gpu.fb.render_full_tilemap(&self.memory);
                self.gpu.fb.update();
                self.gpu.set_mode(GpuMode::VBlank);
            }

            if self.gpu.ly == 154 {
                self.gpu.ly = 0;
                self.gpu.set_mode(GpuMode::OAM);
            }
        }
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

impl GPU {

    pub fn get_mode(&mut self) -> GpuMode {
        let stat = &self.lcd_status;

        if      stat.mode_flag_0 && stat.mode_flag_1    { GpuMode::PixelTransfer }
        else if stat.mode_flag_0                        { GpuMode::VBlank }
        else if stat.mode_flag_1                        { GpuMode::OAM }
        else                                            { GpuMode::HBlank }
    }

    pub fn set_mode(&mut self, mode: GpuMode) {
        match mode {
            GpuMode::OAM => {
                self.lcd_status.mode_flag_0 = false;
                self.lcd_status.mode_flag_1 = true;
            }
            GpuMode::PixelTransfer => {
                self.lcd_status.mode_flag_0 = true;
                self.lcd_status.mode_flag_1 = true;
            }
            GpuMode::HBlank => {
                self.lcd_status.mode_flag_0 = false;
                self.lcd_status.mode_flag_1 = false;
            }
            GpuMode::VBlank => {
                self.lcd_status.mode_flag_0 = true;
                self.lcd_status.mode_flag_1 = false;
            }
        }
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
