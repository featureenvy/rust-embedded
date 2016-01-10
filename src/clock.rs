use memory::{set_value,clear_value,read_value};

const SYSCTL_RCC_R: *mut u32 = 0x400FE060 as *mut u32;
const SYSCTL_RIS_R: *mut u32 = 0x400FE050 as *mut u32;

pub fn init() {
    unsafe {
        set_value(SYSCTL_RCC_R, 0x00000800); // enable bypass
        clear_value(SYSCTL_RCC_R, 0x00400000); // disable sysdiv
        clear_value(SYSCTL_RCC_R, 0x00000001); // enable main OSC
        clear_value(SYSCTL_RCC_R, 0x000007C0); // clear XTAL fields
        set_value(SYSCTL_RCC_R, 0x00000600); // select 20MHz XTAL
        clear_value(SYSCTL_RCC_R, 0x07800000); // clear sysdiv
        set_value(SYSCTL_RCC_R, 0x04800000); // set div to 0x9 (div 10, 20MHz)
        set_value(SYSCTL_RCC_R, 0x00400000); // enable sysdiv
        clear_value(SYSCTL_RCC_R, 0x00000030); // select main oscillator
        clear_value(SYSCTL_RCC_R, 0x00002000); // power up PLL
        while read_value(SYSCTL_RIS_R, 0x00000040) == 0 {}; // wait for PLL
        clear_value(SYSCTL_RCC_R, 0x00000800); // clear bypass and enable PLL
    }
}

pub fn delay(seconds: u64) {
    let mut iterations = seconds * 20_000_00;
    while iterations > 0 {
        unsafe {
            read_value(SYSCTL_RIS_R, 0x40); //dummy read so it is not optimized away
        }
        iterations = iterations - 1;
    }
}
