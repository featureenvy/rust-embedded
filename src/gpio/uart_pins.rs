use gpio::{Port, Pins};
use gpio::gpio_register::GPIORegister;
use memory;

const GPIO_RCGC1_R: *mut u32 = 0x400F_E104 as *mut u32;
const UART_UART0_FR_R: *mut u32 = 0x4000_C018 as *mut u32;
const UART_UART0_IBRD_R: *mut u32 = 0x4000_C024 as *mut u32;
const UART_UART0_FBRD_R: *mut u32 = 0x4000_C028 as *mut u32;
const UART_UART0_LCRH: *mut u32 = 0x4000_C02C as *mut u32;
const UART_UART0_CC: *mut u32 = 0x4000_CFC8 as *mut u32;
const UART_UART0_DR: *mut u32 = 0x4000_C000 as *mut u32;
const UART_CTL_R: *mut u32 = 0x4000_C030 as *mut u32;
const GPIO_RCGC_GPIO_R: *mut u32 = 0x400F_E608 as *mut u32;

pub struct UartPins {
    registers: GPIORegister,
    input_pin: u32,
    output_pin: u32,
}

impl UartPins {
    pub fn new(port: Port, input: Pins, output: Pins) -> UartPins {
        let registers = GPIORegister::new(port);

        let input_value = input as u32;
        let output_value = output as u32;

        unsafe {
            memory::set(GPIO_RCGC1_R, 0x0);
            memory::set(GPIO_RCGC_GPIO_R, 0x0);
            memory::set(registers.afsel_r, input_value);
            memory::set(registers.afsel_r, output_value);
            memory::set(registers.den_r, input_value);
            memory::set(registers.den_r, output_value);
            memory::write(registers.pctl_r, 0x11);

            memory::clear(UART_CTL_R, 0x0); // disable UART
            // baud rate = 20_000_000 / (16 * 9500) = 130.2083
            // integer part: 130 (0x82)
            // divisor part: (0.2083 * 64 + 0.5) = 14 (0x0E)
            memory::write(UART_UART0_IBRD_R, 10);
            memory::write(UART_UART0_FBRD_R, 55);
            memory::write(UART_UART0_LCRH, 0x70); // set to 8bit mode
            memory::write(UART_UART0_CC, 0x0); // use system clock
            memory::set(UART_CTL_R, 0x0); // enable UART0
        }

        UartPins {
            registers: registers,
            input_pin: input_value as u32,
            output_pin: output_value as u32,
        }
    }

    pub fn put(&self, value: u32) {
        unsafe {
            while memory::read(UART_UART0_FR_R, 0x5) != 0 {}
            memory::write(UART_UART0_DR, value);
        }
    }
}
