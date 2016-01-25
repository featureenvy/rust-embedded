use core::str;
use core::array::FixedSizeArray;
use hal::clock;
use driver::{led, uart};
use hal::gpio;
use hal::uart::Uarts;

pub fn run() {
    clock::init();

    let uart = uart::Uart::new(Uarts::Uart0, true);
    let led = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);

    uart.write("\nThis program calculates areas of rectangular rooms.\n");

    loop {
        led.toggle();
        uart.write("\nGive length: ");
        let length = uart.read();

        uart.write("Given length:\n");
        uart.write(str::from_utf8(length.as_slice()).unwrap());
    }
}
