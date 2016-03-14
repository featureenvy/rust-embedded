#![no_std]
#![feature(lang_items, start)]

extern crate rust_embedded;

use core::fmt::Write;

use rust_embedded::hal::clock;
use rust_embedded::driver::{led, uart};
use rust_embedded::hal::gpio;
use rust_embedded::hal::uart::Uarts;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    clock::init();

    let mut uart = uart::Uart::new(Uarts::Uart0, true);
    let led = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);

    uart.write_str("\n\nThis program calculates areas of rectangular rooms.\n").unwrap();

    loop {
        led.toggle();
        uart.write_str("\nGive length: ").unwrap();
        let length = uart.read_int();

        uart.write_str("\nGive width: ").unwrap();
        let width = uart.read_int();

        write!(uart, "\nSize of room: {}", length * width).unwrap();

        uart.write_str("\n\n").unwrap();
    }
}
