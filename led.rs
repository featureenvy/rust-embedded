use gpio;

pub struct Led {
    pin: gpio::DigitalPin,
}

impl Led {
    pub fn new(port: gpio::Port, pin_num: gpio::Pins) -> Led {
        let pin = gpio::DigitalPin::new_output(port, pin_num);
        Led{pin: pin}
    }

    pub fn on(&self) {
        self.pin.set();
    }

    pub fn off(&self) {
        self.pin.clear();
    }

    pub fn toggle(&self) {
        // let mut led = self.pin.read();
        // led = led ^ 0x1;
        // self.pin.write(led);
    }
}
