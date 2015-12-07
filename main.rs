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
mod switch;
pub mod systick;

#[no_mangle]
pub fn main() {
    clock::init();
    let led_red = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);

    let led_blue = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin2);

    let switch = switch::Switch::new(gpio::Port::PortF, gpio::Pins::Pin4);

    led_red.on();
    led_blue.on();

    loop {
        if switch.on() {
            led_red.off();
            led_blue.on();
        } else {
            led_red.on();
            led_blue.off();
        }
    }
}
