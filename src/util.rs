pub const MSB: u8 = 7;
pub const LSB: u8 = 0;

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

    /// Turns a word into (msb, lsb).
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
    /// msb = 00000001, lsb = 01010000
    /// Smush them together with bit shifting to get 0b0000000101010000 == 0x150
    pub fn bytes_to_word(msb: u8, lsb: u8) -> u16 {
        (((msb as u16) << 8) | (lsb as u16)).into()
    }

    /// Get the value of a byte at position n, where 0 is the LSB and 7 is the MSB.
    pub fn get_bit(byte: u8, n: u8) -> bool {
        let result = byte & (1u8 << n);
        result != 0
    }

    /// Set the value of a byte at position n, where 0 is the LSB and 7 is the MSB.
    pub fn set_bit(byte: u8, n: u8) -> u8 {
        let setter = 1u8 << n;
        byte | setter
    }

    /// Reset the value of a byte at position n, where 0 is the LSB and 7 is the MSB.
    pub fn reset_bit(byte: u8, n: u8) -> u8 {
        // e.g. for bit=3 11110111, bit=2 11111011
        let resetter = !(1u8 << n);
        byte & resetter
    }

    /// Add two bytes.
    /// Returns (the result, the half_carry flag, and the carry flag).
    pub fn add_with_flags(left: u8, right: u8) -> (u8, bool, bool) {

        let (result, carry) = left.overflowing_add(right);

        // Add lower nibbles together, and then check if bit 3 is set.
        let half_add = (left & 0b00001111).wrapping_add(right & 0b00001111);
        let half_carry = Util::get_bit(half_add, 4);

        (result, half_carry, carry)
    }

    /// Subtracts two bytes.
    /// Returns (the result, the half_carry flag, and the carry flag).
    pub fn sub_with_flags(left: u8, right: u8) -> (u8, bool, bool) {

        let (result, carry) = left.overflowing_sub(right);

        // Subtract lower nibbles, and then check if bit 3 is set.
        let half_sub = (left & 0b00001111).wrapping_sub(right & 0b00001111);
        let half_carry = Util::get_bit(half_sub, 4);

        (result, half_carry, carry)
    }

    pub fn add_u16_with_flags(left: u16, right: u16) -> (u16, bool, bool) {

        let (result, carry) = left.overflowing_add(right);

        let half_add = (left & 0b0000_1111_1111_1111).wrapping_add(right & 0b0000_1111_1111_1111);
        let half_carry = (half_add & 0b0001_0000_0000_0000) != 0;

        (result, half_carry, carry)
    }

    pub fn sub_u16_with_flags(left: u16, right: u16) -> (u16, bool, bool) {

        let (result, carry) = left.overflowing_sub(right);

        let half_sub = (left & 0b0000_1111_1111_1111).wrapping_sub(right & 0b0000_1111_1111_1111);
        let half_carry = (half_sub & 0b0001_0000_0000_0000) != 0;

        (result, half_carry, carry)
    }

    pub fn add_u16_lower_with_flags(left: u16, right: u16) -> (u16, bool, bool) {

        let result = left.wrapping_add(right);
        let carry = (left & 0x00FF) + (right & 0x00FF) > 0x00FF;
        let half_carry = (left & 0x000F) + (right & 0x000F) > 0x000F;

        (result, half_carry, carry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_flag() {
        assert_eq!(Util::get_bit(0b00010000, 4), true);
        assert_eq!(Util::get_bit(0b00010000, 5), false);
    }

    #[test]
    fn test_set_flag() {
        assert_eq!(Util::set_bit(0b00010000, 0), 0b00010001);
        assert_eq!(Util::set_bit(0b00010000, 4), 0b00010000);
    }

    #[test]
    fn test_reset_flag() {
        assert_eq!(Util::reset_bit(0b00010000, 0), 0b00010000);
        assert_eq!(Util::reset_bit(0b00010000, 4), 0);
    }

    #[test]
    fn add_with_carry_flags() {

        // Easy test with only lower bits.
        let (result, hc, c) = Util::add_with_flags(0b00001010, 0b00001100);
        assert_eq!(result, 0b00010110);
        assert_eq!(hc, true);
        assert_eq!(c, false);

        // Full carry too.
        let (result, hc, c) = Util::add_with_flags(0b11111010, 0b11111100);
        assert_eq!(result, 0b011110110);
        assert_eq!(hc, true);
        assert_eq!(c, true);

        // Only full carry.
        let (result, hc, c) = Util::add_with_flags(0b11110010, 0b11110100);
        assert_eq!(result, 0b011100110);
        assert_eq!(hc, false);
        assert_eq!(c, true);
    }
}
