use hal::{clock, gpio};
use driver::{led, switch};

pub fn run() {
    clock::init();

    let switch = switch::Switch::new(gpio::Port::PortE, gpio::Pins::Pin0);
    let led = led::Led::new(gpio::Port::PortE, gpio::Pins::Pin1);

    led.on();

    loop {
        if switch.is_on() {
            led.toggle();
        } else {
            led.on();
        }

        clock::delay(100);
    }
}
