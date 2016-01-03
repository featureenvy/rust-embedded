pub enum Port {
    PortF = 0x4002_5000,
}

pub enum Pins {
    Pin1 = 1,
    Pin2 = 2,
    Pin3 = 3,
    Pin4 = 4,
}

pub use self::digital_pin::DigitalPin;

mod gpio_register;
mod digital_pin;
