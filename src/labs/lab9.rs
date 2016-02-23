use hal::{clock, gpio};
use driver::{led, switch};

static mut data: [u8; 50] = [0; 50];

pub fn run() {
    clock::init();

    let led = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);
    let switch_one = switch::Switch::new(gpio::Port::PortF, gpio::Pins::Pin4, gpio::Logic::Positive);
    let switch_two = switch::Switch::new(gpio::Port::PortF, gpio::Pins::Pin0, gpio::Logic::Positive);

    led.on();

    let mut index = 0;

    loop {
        if index < 50 {
            unsafe {
               data[index] = 1;
            }
            index = index + 1;
        }

        if switch_two.is_on() {
            led.toggle();
        } else {
            led.off();
        }
        clock::delay(500);
    }

}

