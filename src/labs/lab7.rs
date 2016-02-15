use hal::{clock, gpio};
use driver::{led, switch};

pub fn run() {
    clock::init();

    let arterial_signal = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin3);
    let arterial_switch = switch::Switch::new(gpio::Port::PortF, gpio::Pins::Pin4, gpio::Logic::Positive);

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
