#[derive(Debug)]
#[derive(Default)]
pub struct Flags {
    pub zero: bool,
    pub subtraction: bool,
    pub half_carry: bool,
    pub carry: bool,

    // These 4 bits always read 0 even if written with a 1.
    // _3: bool,
    // _2: bool,
    // _1: bool,
    // _0: bool
}

impl Flags {
    pub fn get_byte(&self) -> u8 {

        let mut result = 0u8;

        if self.zero { result = result | 0b10000000 };
        if self.subtraction { result = result | 0b01000000 };
        if self.half_carry { result = result | 0b00100000 };
        if self.carry { result = result | 0b00010000 };

        result
    }

    pub fn reset(&mut self) {
        self.zero = false;
        self.subtraction = false;
        self.half_carry = false;
        self.carry = false;
    }
}
