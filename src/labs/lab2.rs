use hal::clock;
use core::fmt::Write;
use driver::{led, uart};
use hal::gpio;
use hal::uart::Uarts;

pub fn run() {
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

        write!(uart, "Size of room: {}", length * width).unwrap();
    }
}
