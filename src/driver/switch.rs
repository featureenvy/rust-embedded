use hal::gpio;

pub struct Switch {
    pin: gpio::DigitalPin,
    direction: gpio::Logic,
}

impl Switch {
    pub fn new(port: gpio::Port, pin_num: gpio::Pins, direction: gpio::Logic) -> Switch {
        let pin = gpio::DigitalPin::new_input(port, pin_num);
        Switch {
            pin: pin,
            direction: direction
        }
    }

    pub fn wait_until_on(&self) {
        while self.is_off() { };
    }

    pub fn wait_until_off(&self) {
        while self.is_on() { };
    }

    pub fn is_on(&self) -> bool {
        match self.direction {
            gpio::Logic::Positive => self.pin.read() == 0,
            gpio::Logic::Negative => self.pin.read() != 0,
        }
    }

    pub fn is_off(&self) -> bool {
        !self.is_on()
    }
}
