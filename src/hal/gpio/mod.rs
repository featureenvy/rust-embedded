pub enum Port {
    PortA = 0x4000_4000,
    PortF = 0x4002_5000,
}

pub enum Pins {
    Pin0 = 0,
    Pin1 = 1,
    Pin2 = 2,
    Pin3 = 3,
    Pin4 = 4,
}

pub use self::digital_pin::DigitalPin;
pub use self::uart_pin::init_uart;

mod gpio_register;
mod digital_pin;
mod uart_pin;