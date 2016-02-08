use memory;
use super::{Port, Pins};
use super::port_data::PortData;
use super::gpio_register::GPIORegister;

pub struct DigitalPin {
    registers: GPIORegister,
    pin: u32,
}

impl DigitalPin {
    pub fn new_input(port: Port, pin: Pins) -> DigitalPin {
        let digital_pin = DigitalPin::new(port, pin);
        digital_pin.make_input();

        digital_pin
    }

    pub fn new_output(port: Port, pin: Pins) -> DigitalPin {
        let digital_pin = DigitalPin::new(port, pin);
        digital_pin.make_output();

        digital_pin
    }

    fn new(port: Port, pin: Pins) -> DigitalPin {
        let port_data = PortData::new(port);
        let registers = GPIORegister::new(port_data);
        let pin_value = pin as u32;

        unsafe {
            memory::write(registers.lock_r, 0x4C4F434B);
            memory::set(registers.cr_r, pin_value);
            memory::clear(registers.amsel_r, pin_value);
            memory::clear(registers.pctl_r, pin_value);
            memory::clear(registers.afsel_r, pin_value);
            memory::set(registers.den_r, pin_value);
        }


        DigitalPin{registers: registers, pin: pin_value}
    }

    fn make_output(&self) {
        unsafe {
            memory::set(self.registers.dir_r, self.pin);
            memory::clear(self.registers.pur_r, self.pin);
        }
    }

    fn make_input(&self) {
        unsafe {
            memory::clear(self.registers.dir_r, self.pin);
            // memory::set(self.registers.pur_r, self.pin);
            memory::clear(self.registers.pur_r, self.pin);
        }
    }

    pub fn read(&self) -> u32 {
        unsafe {
            memory::read(self.registers.data_r, self.pin)
        }
    }

    pub fn set(&self) {
        unsafe {
            memory::set(self.registers.data_r, self.pin);
        }
    }

    pub fn toggle(&self, on: bool) {
        if on {
            self.set();
        } else {
            self.clear();
        }
    }

    pub fn clear(&self) {
        unsafe {
            memory::clear(self.registers.data_r, self.pin);
        }
    }
}
