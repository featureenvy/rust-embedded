use hal::gpio;

pub struct Switch {
    pin: gpio::DigitalPin,
}

impl Switch {
    pub fn new(port: gpio::Port, pin_num: gpio::Pins) -> Switch {
        let pin = gpio::DigitalPin::new_input(port, pin_num);
        Switch{pin: pin}
    }

    pub fn wait_until_on(&self) {
        while self.is_off() { };
    }

    pub fn wait_until_off(&self) {
        while self.is_on() { };
    }

    pub fn is_on(&self) -> bool {
        self.pin.read() == 0
    }

    pub fn is_off(&self) -> bool {
        !self.is_on()
    }
}
