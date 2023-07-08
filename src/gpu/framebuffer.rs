use minifb::{Scale, Window, WindowOptions};
use crate::memory::memory::Memory;
use crate::util::Util;

const WIDTH: usize = 192;
const HEIGHT: usize = 128;

#[derive(Debug)]
pub struct FrameBuffer {
    buffer: [u32; WIDTH * HEIGHT],
    window: Window
}

struct Pixel {
    r: u8,
    b: u8,
    g: u8
}

impl FrameBuffer {
    pub fn update(&mut self) {
        self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap()
    }

    pub fn render_full_tilemap(&mut self, memory: &Memory) {

        // println!("{:?}", memory.buffer[0x8000..0x8010].iter());

        for (tile_idx, chunk) in memory.buffer[0x8000..0x9800].chunks(16).enumerate() {

            let mut tile = [0u32; 8*8];

            for (line_idx, line) in chunk.chunks(2).enumerate() {
                for i in 0..8 {
                    let lsb = Util::get_bit(line[0], i);
                    let msb = Util::get_bit(line[1], i);
                    let pixel = FrameBuffer::get_pixel(lsb, msb);
                    tile[(line_idx * 8) + i as usize] = pixel;
                }
            }

            self.buffer[(tile_idx * 64)..(tile_idx * 64) + 64].copy_from_slice(tile.as_slice());
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
        let mut window = Window::new(
            "Test - ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions { scale: Scale::X4, ..WindowOptions::default() }
        ).unwrap();
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        return FrameBuffer {
            buffer: [0; WIDTH * HEIGHT],
            window
        }
    }
}