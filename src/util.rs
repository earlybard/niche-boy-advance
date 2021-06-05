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

    pub fn get_flag(byte: u8, bit: u8) -> bool {
        let result = byte & (1u8 << bit);
        result != 0
    }

    pub fn set_flag(byte: u8, bit: u8) -> u8 {
        let setter = 1u8 << bit;
        byte | setter
    }

    pub fn reset_flag(byte: u8, bit: u8) -> u8 {
        // e.g. for bit=3 11110111, bit=2 11111011
        let resetter = !(1u8 << bit);
        byte & resetter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_flag() {
        assert_eq!(Util::get_flag(0b00010000, 4), true);
        assert_eq!(Util::get_flag(0b00010000, 5), false);
    }

    #[test]
    fn test_set_flag() {
        assert_eq!(Util::set_flag(0b00010000, 0), 0b00010001);
        assert_eq!(Util::set_flag(0b00010000, 4), 0b00010000);
    }

    #[test]
    fn test_reset_flag() {
        assert_eq!(Util::reset_flag(0b00010000, 0), 0b00010000);
        assert_eq!(Util::reset_flag(0b00010000, 4), 0);
    }
}
