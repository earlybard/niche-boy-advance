use minifb::{Scale, Window, WindowOptions};
use crate::memory::memory::Memory;
use crate::util::Util;

const WIDTH: usize = 192;
const HEIGHT: usize = 128;

#[derive(Debug)]
pub struct FrameBuffer {
    buffer: [u32; WIDTH * HEIGHT],
    window: Option<Window>,
    frame_skip_counter: usize
}

struct Pixel {
    r: u8,
    b: u8,
    g: u8
}

impl FrameBuffer {
    pub fn enable_window(&mut self) {
        let mut window = Window::new(
            "Test - ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions { scale: Scale::X4, ..WindowOptions::default() }
        ).unwrap();
        // window.limit_update_rate(Some(std::time::Duration::from_micros(1660000)));

        self.window = Some(window);
    }

    pub fn update(&mut self) {
        if let Some(ref mut w) = self.window {
            if self.frame_skip_counter == 256 {
                w.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();
                self.frame_skip_counter = 0;
            } else {
                self.frame_skip_counter += 1;
            }
        }
    }

    pub fn render_full_tilemap(&mut self, memory: &Memory) {

        let mut counter = 0;

        // Each tile occupies 16 bytes
        for (tile_idx, chunk) in memory.buffer[0x8000..0x9800].chunks(16).enumerate() {

            let mut tile = [0u32; 8*8];

            // println!("");
            // Each line is 2 of the 16 bytes.
            for (line_idx, line) in chunk.chunks(2).enumerate() {

                // print!("{:02X} {:02X} ", line[0], line[1]);
                for i in 0..8 {
                    let lsb = Util::get_bit(line[1], 7 - i);
                    let msb = Util::get_bit(line[0], 7 - i);
                    let pixel = FrameBuffer::get_pixel(lsb, msb);
                    tile[(line_idx * 8) + i as usize] = pixel;
                }
            }

            let row = (tile_idx * 8) / WIDTH;
            let col = (tile_idx * 8) % WIDTH;
            let start = (row * 8 * WIDTH) + col;

            for (i, line) in tile.chunks(8).enumerate() {
                let offset = start + (i * WIDTH);
                self.buffer[offset .. offset + 8].copy_from_slice(line);
            }

            // Mark out tiles
            // self.buffer[start] = FrameBuffer::from_u8_rgb(255, 0 ,0);

            // break;
        }
    }

    fn get_pixel(lsb: bool, msb: bool) -> u32 {
        if lsb && msb { FrameBuffer::from_u8_rgb(255, 255, 255) }
        else if msb { FrameBuffer::from_u8_rgb(180, 180, 180) }
        else if lsb { FrameBuffer::from_u8_rgb(100, 100, 100) }
        else { FrameBuffer::from_u8_rgb(0, 0, 0) }
    }

    fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }
}


impl Default for FrameBuffer {

    fn default() -> Self {
        return FrameBuffer {
            buffer: [0; WIDTH * HEIGHT],
            window: None,
            frame_skip_counter: 0,
        }
    }
}