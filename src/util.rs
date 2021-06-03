pub struct Util {}

#[allow(dead_code)]
impl Util {
    pub fn print_binary_u8(input: u8) {
        eprintln!("{:#010b}", input);
    }

    pub fn print_binary_u16(input: u16) {
        eprintln!("{:#018b}", input);
    }

    // Turn a byte into the lsb of a word.
    pub fn byte_to_word(byte: u8) -> u16 {
        Util::bytes_to_word(0, byte)
    }

    pub fn word_to_bytes(word: u16) -> (u8, u8) {
        (Util::get_msb(word), Util::get_lsb(word))
    }

    pub fn get_lsb(word: u16) -> u8 {
        (word & 0x00FF) as u8
    }

    pub fn get_msb(word: u16) -> u8 {
        ((word & 0xFF00) >> 8) as u8
    }

    /// Combine two bytes into a word.
    pub fn bytes_to_word(msb: u8, lsb: u8) -> u16 {
        (((msb as u16) << 8) | (lsb as u16)).into()
    }
}
