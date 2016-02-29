use hal::{clock, gpio};
use driver::{led, switch};

pub fn run() {
    clock::init();

    let pedestrian_red = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);
    let pedestrian_green = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin0);
    let pedestrian_waiting = switch::Switch::new(gpio::Port::PortE, gpio::Pins::Pin2, gpio::Logic::Negative);

    let west_red = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin5);
    let west_yellow = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin4);
    let west_green = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin3);
    let west_waiting = switch::Switch::new(gpio::Port::PortE, gpio::Pins::Pin0, gpio::Logic::Negative);

    let north_red = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin2);
    let north_yellow = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin1);
    let north_green = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin0);
    let north_waiting = switch::Switch::new(gpio::Port::PortE, gpio::Pins::Pin1, gpio::Logic::Negative);

    north_red.off();
    north_yellow.off();
    north_green.off();
    west_red.off();
    west_yellow.off();
    west_green.off();
    pedestrian_red.off();
    pedestrian_green.off();

    loop {
        if west_waiting.is_on() {
            west_red.on();
        } else {
            west_red.off();
        }

        if pedestrian_waiting.is_on() {
            west_green.on();
        } else {
            west_green.off();
        }

        if north_waiting.is_on() {
            north_red.on();
        } else {
            north_red.off();
        }
    }
}
