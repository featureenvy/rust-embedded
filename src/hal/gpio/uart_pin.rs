use super::{Port, Pins};
use super::port_data::PortData;
use super::gpio_register::GPIORegister;
use memory;

const GPIO_RCGC_GPIO_R: *mut u32 = 0x400F_E608 as *mut u32;

pub fn init_uart(port: Port, input: Pins, output: Pins) {
    let port_data = PortData::new(port);
    let gpio_register = GPIORegister::new(port_data);

    let input_value = input as u32;
    let output_value = output as u32;

    unsafe {
        memory::set(GPIO_RCGC_GPIO_R, port_data.position);
        memory::set(gpio_register.afsel_r, input_value);
        memory::set(gpio_register.afsel_r, output_value);
        memory::set(gpio_register.den_r, input_value);
        memory::set(gpio_register.den_r, output_value);
        memory::write(gpio_register.pctl_r, 0x11);
    }
}
