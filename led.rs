use gpio;

pub fn init() {
    let port_f = gpio::GPIO::new(gpio::Port::PORTF);
    port_f.make_output();
}

pub fn toggle() {
    let mut led = gpio::read(gpio::Port::PORTF, 0x02);
    led = led ^ 0x02;
    gpio::write(gpio::Port::PORTF, led);
}
