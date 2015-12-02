#![feature(no_std)]
#![feature(core)]

#![no_std]
#![crate_type="staticlib"]
#![feature(lang_items)]
#![feature(core_intrinsics)]

mod runtime;
mod memory;
mod clock;
mod gpio;
mod led;
pub mod systick;

#[no_mangle]
pub fn main() {
    clock::init();
    let led_red = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);

    let led_blue = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin2);

    led_red.on();
    //led_red.off();

    led_blue.on();
    //led_blue.off();

   loop {}

}
