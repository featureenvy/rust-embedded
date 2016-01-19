use hal::clock;
use driver::{led, uart};
use hal::gpio;
use hal::uart::Uarts;

pub fn run() {
    clock::init();

    let uart = uart::Uart::new(Uarts::Uart0);
    let led = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);

    loop {
        led.toggle();
        let value = uart.read();
        uart.write(value);
    }
}
