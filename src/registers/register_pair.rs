use crate::util::Util;

#[derive(Debug)]
#[derive(Default)]
pub struct RegisterPair {
    pub first: u8,
    pub second: u8
}

impl RegisterPair {
    pub fn get_word(&self) -> u16 {
        // Util::print_binary_u8(self.hi);
        // Util::print_binary_u8(self.lo);
        Util::bytes_to_word(self.first, self.second)
    }
    pub fn set_word(&mut self, word: u16) {
        let (first, second) = Util::word_to_bytes(word);
        self.first = first;
        self.second = second;
    }
}
