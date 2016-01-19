use hal::gpio;

pub struct Switch {
    pin: gpio::DigitalPin,
}

impl Switch {
    pub fn new(port: gpio::Port, pin_num: gpio::Pins) -> Switch {
        let pin = gpio::DigitalPin::new_input(port, pin_num);
        Switch{pin: pin}
    }

    pub fn on(&self) -> bool {
        self.pin.read() == 0
    }
}
