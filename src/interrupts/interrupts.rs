use crate::flags_byte;

#[derive(Debug)]
pub struct Interrupts {
    pub ime: bool,
    pub interrupt_enable: InterruptEnable,
    pub interrupt_flag: InterruptFlag
}

impl Default for Interrupts {
    fn default() -> Self {
        Interrupts {
            ime: false,
            interrupt_enable: InterruptEnable::default(),
            interrupt_flag: InterruptFlag {
                _7: true,
                _6: true,
                _5: true,
                joypad: false,
                serial: false,
                timer: false,
                lcdc: false,
                vblank: false,
            }
        }
    }
}

// Interrupts enabled
flags_byte!(InterruptEnable,
    _7,
    _6,
    _5,
    joypad,
    serial,
    timer,
    lcdc,
    vblank
);

// Interrupt requests
flags_byte!(InterruptFlag,
    _7,
    _6,
    _5,
    joypad,
    serial,
    timer,
    lcdc,
    vblank
);