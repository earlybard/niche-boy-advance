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