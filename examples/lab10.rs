#![no_std]
#![feature(lang_items, start)]

extern crate rust_embedded;

mod lab10_code;

use rust_embedded::hal::clock;
use rust_embedded::driver::led;
use rust_embedded::driver::switch;
use rust_embedded::hal::gpio;

use lab10_code::signal::Signal;
use lab10_code::pedestrian_signal::PedestrianSignal;
use lab10_code::signaling_state::State;
use lab10_code::signaling_state::ALL_RED_REF;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    clock::init();

    let pedestrian_red = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);
    let pedestrian_green = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin3);
    let pedestrian_waiting = switch::Switch::new(gpio::Port::PortE, gpio::Pins::Pin2, gpio::Logic::Negative);

    let pedestrian_signal = PedestrianSignal::new(pedestrian_red, pedestrian_green);

    let west_red = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin5);
    let west_yellow = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin4);
    let west_green = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin3);
    let west_waiting = switch::Switch::new(gpio::Port::PortE, gpio::Pins::Pin0, gpio::Logic::Negative);

    let west_signal = Signal::new(west_red, west_yellow, west_green);

    let north_red = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin2);
    let north_yellow = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin1);
    let north_green = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin0);
    let north_waiting = switch::Switch::new(gpio::Port::PortE, gpio::Pins::Pin1, gpio::Logic::Negative);

    let north_signal = Signal::new(north_red, north_yellow, north_green);

    let mut signaling_system: &'static State = &ALL_RED_REF;

    loop {
        let signal_values = signaling_system.get_signal_values();

        pedestrian_signal.set(signal_values.pedestrian);
        west_signal.set(signal_values.west);
        north_signal.set(signal_values.north);

        clock::delay(signaling_system.wait_duration());

        signaling_system = signaling_system.next(pedestrian_waiting.is_on(), west_waiting.is_on(), north_waiting.is_on());
    }
}
