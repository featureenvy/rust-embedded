use hal::clock;
use driver::led;
use driver::switch;
use hal::gpio;

static mut data: [u8; 50] = [0; 50];

#[allow(dead_code)]
pub fn run() {
    clock::init();

    let led = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);
    let switch_one = switch::Switch::new_internal(switch::InternalSwitch::Switch0);
    let switch_two = switch::Switch::new_internal(switch::InternalSwitch::Switch1);

    led.on();

    let mut index = 0;

    loop {
        if index < 50 {
            unsafe {
               data[index] = 1;
            }
            index = index + 1;
        }

        if switch_two.is_on() || switch_one.is_on() {
            led.toggle();
        } else {
            led.off();
        }
        clock::delay(500);
    }

}

