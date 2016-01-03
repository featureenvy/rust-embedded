use memory::{set_value,clear_value,read_value,write};

const SYSCTL_RCGC2_R: *mut u32 = 0x400FE108 as *mut u32;
const SYSCTL_RCC_R: *mut u32 = 0x400FE060 as *mut u32;
const SYSCTL_RCC2_R: *mut u32 = 0x400FE070 as *mut u32;
const SYSCTL_RIS_R: *mut u32 = 0x400FE050 as *mut u32;

pub fn init() {
    unsafe {
        set_value(SYSCTL_RCC_R, 0x00000800); // enable bypass
        clear_value(SYSCTL_RCC_R, 0x00400000); // disable sysdiv
        clear_value(SYSCTL_RCC_R, 0x00000001); // enable main OSC
        while read_value(SYSCTL_RIS_R, 0x00000100) == 0 {}; // wait for main OSC to start
        clear_value(SYSCTL_RCC_R, 0x000007C0); // clear XTAL fields
        set_value(SYSCTL_RCC_R, 0x00000540); // select 16MHz XTAL
        clear_value(SYSCTL_RCC_R, 0x00000030); // select main oscillator
        clear_value(SYSCTL_RCC_R, 0x00002000); // power up PLL

        clear_value(SYSCTL_RCC_R, 0x07800000); // clear sysdiv
        set_value(SYSCTL_RCC_R, 0x01000000); // set div to 4 (50MHz)
        set_value(SYSCTL_RCC_R, 0x00400000); // enable sysdiv
        while read_value(SYSCTL_RIS_R, 0x00000040) == 0 {}; // wait for PLL
        clear_value(SYSCTL_RCC_R, 0x00000800);
        write(SYSCTL_RCC2_R, 0xc1004000);

        set_value(SYSCTL_RCGC2_R, 0x00000020);
        read_value(SYSCTL_RCGC2_R, 0x1);
    }
}

pub fn delay(seconds: u64) {
    let mut iterations = seconds * 5000;
    while iterations > 0 {
        unsafe {
            read_value(SYSCTL_RIS_R, 0x40); //dummy read so it is not optimized away
        }
        iterations = iterations - 1;
    }
}
