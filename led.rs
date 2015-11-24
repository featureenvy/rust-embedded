use memory::{set,read,write};


const GPIO_PORTF_CR_R: *mut u32 = 0x40025524 as *mut u32;
const GPIO_PORTF_AMSEL_R: *mut u32 = 0x40025528 as *mut u32;
const GPIO_PORTF_DIR_R: *mut u32 = 0x40025400 as *mut u32;
const GPIO_PORTF_AFSEL_R: *mut u32 = 0x40025420 as *mut u32;
const GPIO_PORTF_PUR_R: *mut u32 = 0x40025510 as *mut u32;
const GPIO_PORTF_DEN_R: *mut u32 = 0x4002551C as *mut u32;
const GPIO_PORTF_DATA_R: *mut u32 = 0x400253FC as *mut u32;
const GPIO_PORTF_PCTL_R: *mut u32 = 0x4002552C as *mut u32;
const GPIO_RCGC_GPIO_R: *mut u32 = 0x400FE608 as *mut u32;

pub fn init() {
    set(GPIO_RCGC_GPIO_R, 0x20);
    write(GPIO_PORTF_CR_R, 0x1F);
    write(GPIO_PORTF_AMSEL_R, 0x00);
    write(GPIO_PORTF_PCTL_R, 0x00000000);
    write(GPIO_PORTF_DIR_R, 0x0E);
    write(GPIO_PORTF_AFSEL_R, 0x00);
    write(GPIO_PORTF_PUR_R, 0x11);
    write(GPIO_PORTF_DEN_R, 0x1F);

    set(GPIO_PORTF_DATA_R, 0x2);
}

pub fn toggle() {
    let mut led = read(GPIO_PORTF_DATA_R, 0x2);
    led = led ^ 0x02;
    write(GPIO_PORTF_DATA_R, led);

}
