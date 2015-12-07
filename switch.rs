use gpio;

pub struct Switch {
    pin: gpio::DigitalPin,
}

impl Switch {
    pub fn new(port: gpio::Port, pin_num: gpio::Pins) -> Switch {
        let pin = gpio::DigitalPin::new(port, pin_num);
        pin.make_input();
        Switch{pin: pin}
    }

    pub fn on(&self) -> bool {
        self.pin.read() == 0
    }
}
