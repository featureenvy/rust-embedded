use clock;
use led;
use switch;
use gpio;

pub fn run() {
    clock::init();

    let led_red = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);
    let led_blue = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin2);
    let led_green = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin3);

    let switch = switch::Switch::new(gpio::Port::PortF, gpio::Pins::Pin4);

    led_red.off();
    led_blue.off();
    led_green.off();

    loop {
        if switch.on() {
            led_blue.off();
            led_red.off();
            led_green.on();
        } else {
            led_blue.off();
            led_red.on();
            led_green.off();
        }
        clock::delay(1000);
        led_red.off();
        led_green.off();
        led_blue.on();
        clock::delay(1000);
    }

}
