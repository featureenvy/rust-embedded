use memory::{set,set_value,clear,clear_value,read_value};

const SYSCTL_RCC_R: *mut u32 = 0x400FE060 as *mut u32;
const SYSCTL_RIS_R: *mut u32 = 0x400FE050 as *mut u32;
const SYSCTL_RCGC2_R: *mut u32 = 0x400FE108 as *mut u32;

pub fn init() {
    unsafe {
        set(SYSCTL_RCC_R, 11); //enable bypass
        clear(SYSCTL_RCC_R, 0); // enable MOSC
        clear(SYSCTL_RCC_R, 22); // disable sysdiv
        clear_value(SYSCTL_RCC_R, 0x000007C0); // clear XTAL fields
        set_value(SYSCTL_RCC_R, 0x00000540); // select 16MHz XTAL
        clear_value(SYSCTL_RCC_R, 0x00000030); // select main oscillator
        clear(SYSCTL_RCC_R, 13); // power up PLL
        clear_value(SYSCTL_RCC_R, 0x07800000); // clear sysdiv
        set_value(SYSCTL_RCC_R, 0x04800000); // set div to 0x9 (div 10, 20MHz)
        set(SYSCTL_RCC_R, 22); // enable sysdiv
        while read_value(SYSCTL_RIS_R, 0x00000040) == 0 {}; // wait for PLL

        set_value(SYSCTL_RCGC2_R, 0x00000020); // enable clock on port F
        clear(SYSCTL_RCC_R, 11); // clear bypass and enable PLL
    }
}

pub fn delay(seconds: u64) {
    let mut iterations = seconds * 2000;
    while iterations > 0 {
        unsafe {
            read_value(SYSCTL_RCC_R, 0x2); //dummy read so it is not optimized away
        }
        iterations = iterations - 1;
    }
}
