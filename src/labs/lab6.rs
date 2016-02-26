use hal::gpio;
use hal::clock;
use driver::{led, switch};

pub fn run() {
    clock::init();

    let led_blue = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin2);
    let switch_one = switch::Switch::new_internal(switch::InternalSwitch::Switch0);

    led_blue.on();

    loop {
        if switch_one.is_on() {
            led_blue.toggle();
            clock::delay(100);
        } else {
            led_blue.on();
        }
    }

}
