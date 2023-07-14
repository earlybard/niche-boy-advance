use crate::bitset;

bitset!(LCDControl,
    0xFF40,
    lcd_enable,
    window_tile_map_area,
    window_enable,
    tile_data_area,
    bg_tile_map_area,
    obj_size,
    obj_enable,
    background_priority
);

bitset!(LCDStatus,
    0xFF41,
    _msb,
    ly_lyc_interrupt,
    mode_2_interrupt,
    mode_1_interrupt,
    mode_0_interrupt,
    ly_equals_lyc,
    mode_flag_1,
    mode_flag_0
);


// Interrupts enabled
bitset!(InterruptEnable,
    0xFFFF,
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
bitset!(InterruptFlag,
    0xFF0F,
    _7,
    _6,
    _5,
    joypad,
    serial,
    timer,
    lcdc,
    vblank
);