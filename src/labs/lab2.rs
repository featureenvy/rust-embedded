use clock;
use uart;
use led;
use gpio;

pub fn run() {
    clock::init();

    let uart = uart::Uart::new(uart::UartDevices::Uart0);
    let led = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);
    let mut count = 33;

    loop {
        led.toggle();
        uart.write(count);
        count = count + 1;
        if count > 126 {
            count = 33;
        }
        clock::delay(100);
    }
}
