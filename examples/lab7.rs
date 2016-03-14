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

    let arterial_signal = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin3);
    let arterial_switch = switch::Switch::new_internal(switch::InternalSwitch::Switch0);

    let ventrical_signal = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);

    arterial_signal.on();

    loop {
        arterial_switch.wait_until_on();
        arterial_signal.off();

        arterial_switch.wait_until_off();
        clock::delay(250);

        ventrical_signal.on();
        clock::delay(250);

        ventrical_signal.off();
        arterial_signal.on();
    }
}
