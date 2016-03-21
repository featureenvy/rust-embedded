use hal::clock;
use driver::led;
use driver::switch;
use hal::gpio;

#[allow(dead_code)]
pub fn run() {
    clock::init();

    let arterial_signal = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin3);
    let arterial_switch = switch::Switch::new_internal(switch::InternalSwitch::Switch0);

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
