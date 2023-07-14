/// This used to be used in place of bitset, now it's only used for flags.
/// The macro is no longer needed but keeping in case there is another use.
#[macro_export]
macro_rules! flags_byte {
    ($struct_name:ident, $msb:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $lsb:ident) => {

        #[derive(Debug)]
        #[derive(Default)]
        pub struct $struct_name {
            pub $msb: bool,
            pub $b: bool,
            pub $c: bool,
            pub $d: bool,
            pub $e: bool,
            pub $f: bool,
            pub $g: bool,
            pub $lsb: bool
        }

        #[allow(dead_code)]
        impl $struct_name {

            pub fn get_byte(&self) -> u8 {

                let mut result = 0u8;

                if self.$msb { result |= 0b10000000 };
                if self.$b   { result |= 0b01000000 };
                if self.$c   { result |= 0b00100000 };
                if self.$d   { result |= 0b00010000 };
                if self.$e   { result |= 0b00001000 };
                if self.$f   { result |= 0b00000100 };
                if self.$g   { result |= 0b00000010 };
                if self.$lsb { result |= 0b00000001 };

                result
            }

            pub fn set_byte(&mut self, byte: u8) {
                self.$msb = byte & 0b10000000 != 0;
                self.$b   = byte & 0b01000000 != 0;
                self.$c   = byte & 0b00100000 != 0;
                self.$d   = byte & 0b00010000 != 0;
                self.$e   = byte & 0b00001000 != 0;
                self.$f   = byte & 0b00000100 != 0;
                self.$g   = byte & 0b00000010 != 0;
                self.$lsb = byte & 0b00000001 != 0;
            }

            pub fn reset(&mut self) {
                self.$msb = false;
                self.$b   = false;
                self.$c   = false;
                self.$d   = false;
                self.$e   = false;
                self.$f   = false;
                self.$g   = false;
                self.$lsb = false;
            }
        }
    }
}

/// Expose bit-level access to a byte in Memory.
#[macro_export]
macro_rules! bitset {
    ($name:ident, $addr:literal, $msb:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $lsb:ident) => {

#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod $name {
    use crate::memory::memory::Memory;
    pub const ADDR: u16 = $addr;

    pub fn get(m: &Memory) -> u8 { m[ADDR] }

    pub fn set(m: &mut Memory, byte: u8) { m[ADDR] = byte }

    pub fn print(m: &Memory) {
        println!("{}", m[ADDR])
    }

    pub mod get {
        use super::*;
        pub fn $msb(m: &Memory) -> bool { m[ADDR] &  0b10000000 != 0 }
        pub fn $b  (m: &Memory) -> bool { m[ADDR] &  0b01000000 != 0 }
        pub fn $c  (m: &Memory) -> bool { m[ADDR] &  0b00100000 != 0 }
        pub fn $d  (m: &Memory) -> bool { m[ADDR] &  0b00010000 != 0 }
        pub fn $e  (m: &Memory) -> bool { m[ADDR] &  0b00001000 != 0 }
        pub fn $f  (m: &Memory) -> bool { m[ADDR] &  0b00000100 != 0 }
        pub fn $g  (m: &Memory) -> bool { m[ADDR] &  0b00000010 != 0 }
        pub fn $lsb(m: &Memory) -> bool { m[ADDR] &  0b00000001 != 0 }
    }

    pub mod put {
        use super::*;
        pub fn $msb(m: &mut Memory, value: bool) { if value { super::set::$msb(m) } else { super::res::$msb(m) } }
        pub fn $b  (m: &mut Memory, value: bool) { if value { super::set::$b  (m) } else { super::res::$b  (m) } }
        pub fn $c  (m: &mut Memory, value: bool) { if value { super::set::$c  (m) } else { super::res::$c  (m) } }
        pub fn $d  (m: &mut Memory, value: bool) { if value { super::set::$d  (m) } else { super::res::$d  (m) } }
        pub fn $e  (m: &mut Memory, value: bool) { if value { super::set::$e  (m) } else { super::res::$e  (m) } }
        pub fn $f  (m: &mut Memory, value: bool) { if value { super::set::$f  (m) } else { super::res::$f  (m) } }
        pub fn $g  (m: &mut Memory, value: bool) { if value { super::set::$g  (m) } else { super::res::$g  (m) } }
        pub fn $lsb(m: &mut Memory, value: bool) { if value { super::set::$lsb(m) } else { super::res::$lsb(m) } }
    }

    pub mod set {
        use super::*;
        pub fn $msb(m: &mut Memory)     { m[ADDR] |= 0b10000000 }
        pub fn $b  (m: &mut Memory)     { m[ADDR] |= 0b01000000 }
        pub fn $c  (m: &mut Memory)     { m[ADDR] |= 0b00100000 }
        pub fn $d  (m: &mut Memory)     { m[ADDR] |= 0b00010000 }
        pub fn $e  (m: &mut Memory)     { m[ADDR] |= 0b00001000 }
        pub fn $f  (m: &mut Memory)     { m[ADDR] |= 0b00000100 }
        pub fn $g  (m: &mut Memory)     { m[ADDR] |= 0b00000010 }
        pub fn $lsb(m: &mut Memory)     { m[ADDR] |= 0b00000001 }
    }

    pub mod res {
        use super::*;
        pub fn $msb(m: &mut Memory)     { m[ADDR] &= 0b01111111 }
        pub fn $b  (m: &mut Memory)     { m[ADDR] &= 0b10111111 }
        pub fn $c  (m: &mut Memory)     { m[ADDR] &= 0b11011111 }
        pub fn $d  (m: &mut Memory)     { m[ADDR] &= 0b11101111 }
        pub fn $e  (m: &mut Memory)     { m[ADDR] &= 0b11110111 }
        pub fn $f  (m: &mut Memory)     { m[ADDR] &= 0b11111011 }
        pub fn $g  (m: &mut Memory)     { m[ADDR] &= 0b11111101 }
        pub fn $lsb(m: &mut Memory)     { m[ADDR] &= 0b11111110 }
    }
}}}