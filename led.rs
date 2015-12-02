use gpio;

pub struct Led {
    pin: gpio::Pin,
}

impl Led {
    pub fn new(port: gpio::Port, pin_num: gpio::Pins) -> Led {
        let pin = gpio::Pin::new(port, pin_num);
        pin.make_output();
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
