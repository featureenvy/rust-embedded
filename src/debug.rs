use driver;
use hal::gpio;

#[allow(dead_code)]
pub fn red_on() {
    let red_led = driver::led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);
    red_led.on();
}

#[allow(dead_code)]
pub fn red_off() {
    let red_led = driver::led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);
    red_led.off();
}
