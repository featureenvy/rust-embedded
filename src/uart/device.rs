use super::uart_register::UartRegister;
use super::Uarts;
use gpio::{Port, Pins};
use gpio;
use memory;

const GPIO_RCGC1_R: *mut u32 = 0x400F_E104 as *mut u32;

pub struct Device {
    register: UartRegister,
}

impl Device {
    pub fn new(uart: Uarts) -> Device {
         match uart {
            Uarts::Uart0 => Device::init_uart(0x4000_C000, Port::PortA, Pins::Pin0, Pins::Pin1),
        }
    }

    fn init_uart(address:u32, port: Port, input: Pins, output: Pins) -> Device {
        let uart_register = UartRegister::new(address);

        unsafe {
            memory::set(GPIO_RCGC1_R, 0x0);

            gpio::init_uart(port, input, output);

            memory::clear(uart_register.ctl_r, 0x0); // disable UART
            // baud rate = 20_000_000 / (16 * 9500) = 130.2083
            // integer part: 130 (0x82)
            // divisor part: (0.2083 * 64 + 0.5) = 14 (0x0E)
            memory::write(uart_register.ibrd_r, 10);
            memory::write(uart_register.fbrd_r, 55);
            memory::write(uart_register.lcrh_r, 0x70); // set to 8bit mode
            memory::write(uart_register.cc_r, 0x0); // use system clock
            memory::set(uart_register.ctl_r, 0x0); // enable UART0

        }

        Device {
            register: uart_register,
        }

    }

    pub fn put(&self, value: u32) {
        unsafe {
            while memory::read(self.register.fr_r, 0x5) != 0 {}
            memory::write(self.register.dr_r, value);
        }
    }

    pub fn read_char(&self) -> u32 {
        unsafe {
            while memory::read(self.register.fr_r, 0x4) != 0 {}
            memory::read_value(self.register.dr_r, 0xFF)
        }
    }
}
