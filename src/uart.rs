use gpio;

#[derive(PartialEq, Eq)]
pub enum UartDevices {
    Uart0,
}

pub struct Uart {
    uart_pins: gpio::UartPins,
}

impl Uart {
    pub fn new(uart: UartDevices) -> Uart {
        match uart {
            UartDevices::Uart0 => Uart {
                uart_pins: gpio::UartPins::new(gpio::Port::PortA, gpio::Pins::Pin0, gpio::Pins::Pin1),
            }

        }
    }

    pub fn write(&self, value: u32) {
        self.uart_pins.put(value);
    }
}
