pub struct UartRegister {
    pub dr_r: *mut u32,
    pub fr_r: *mut u32,
    pub ibrd_r: *mut u32,
    pub fbrd_r: *mut u32,
    pub lcrh_r: *mut u32,
    pub ctl_r: *mut u32,
    pub cc_r: *mut u32,
}

impl UartRegister {
    pub fn new(uart: u32) -> UartRegister {
        UartRegister {
            dr_r: (uart + 0x000) as *mut u32,
            fr_r: (uart + 0x018) as *mut u32,
            ibrd_r: (uart + 0x024) as *mut u32,
            fbrd_r: (uart + 0x028) as *mut u32,
            lcrh_r: (uart + 0x02C) as *mut u32,
            ctl_r: (uart + 0x030) as *mut u32,
            cc_r: (uart + 0xFC8) as *mut u32,
        }
    }
}
