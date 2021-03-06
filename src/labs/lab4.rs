use hal::clock;
use driver::led;
use driver::switch;
use hal::gpio;

#[allow(dead_code)]
pub fn run() {
    clock::init();

    let led_red = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);
    let led_blue = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin2);
    let led_green = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin3);

    let switch_one = switch::Switch::new_internal(switch::InternalSwitch::Switch0);
    let switch_two = switch::Switch::new_internal(switch::InternalSwitch::Switch1);

    led_red.on();
    led_blue.on();
    led_green.on();

    loop {
        if switch_one.is_on() && switch_two.is_on() {
            led_red.off();
            led_blue.on();
            led_green.off();
        } else if switch_one.is_on() {
            led_red.on();
            led_blue.off();
            led_green.off();
        } else if switch_two.is_on() {
            led_red.off();
            led_blue.off();
            led_green.on();
        } else {
            led_red.off();
            led_blue.off();
            led_green.off();
        }
    }

}
