use clock;
use uart;
use led;
use gpio;

pub fn run() {
    clock::init();

    let uart = uart::Uart::new(uart::Uarts::Uart0);
    let led = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);

    loop {
        led.toggle();
        let value = uart.read();
        uart.write(value);
    }
}
