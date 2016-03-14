#![no_std]
#![feature(lang_items, start)]

extern crate rust_embedded;

use rust_embedded::hal::clock;
use rust_embedded::driver::led;
use rust_embedded::driver::switch;
use rust_embedded::hal::gpio;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    clock::init();

    let led_blue = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin2);
    let switch_one = switch::Switch::new_internal(switch::InternalSwitch::Switch0);

    led_blue.on();

    loop {
        if switch_one.is_on() {
            led_blue.toggle();
            clock::delay(100);
        } else {
            led_blue.on();
        }
    }

}
