pub fn dma(buffer: &mut [u8], byte: u8) {
    // DMA transfer
    let start = (byte as usize) << 8;

    // Copy dma .. dma + 0xA0
    // to 0xFE00..0xFEA0
    for i in 0..0xA0 {
        buffer[0xFE00 + i] = buffer[start + i]
    }

    // Fancier solution
    // let (left, right) = self.memory.buffer.split_at_mut(0xFE00);
    // right[0x00..0xA0].copy_from_slice(&left[dma..(dma + 0xA0)]);
}