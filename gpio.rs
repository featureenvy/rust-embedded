use memory;

const GPIO_RCGC_GPIO_R: *mut u32 = 0x400FE608 as *mut u32;

pub enum Port {
    PortF = 0x4002_5000,
}

pub enum Pins {
    Pin1 = 1,
    Pin2 = 2,
    Pin3 = 3,
    Pin4 = 4,
}

struct GPIORegister {
    cr_r: *mut u32,
    amsel_r: *mut u32,
    pctl_r: *mut u32,
    dir_r: *mut u32,
    afsel_r: *mut u32,
    pur_r: *mut u32,
    den_r: *mut u32,
    data_r: *mut u32,
}

impl GPIORegister {
    fn new(port_name: Port) -> GPIORegister {
        let port = port_name as u32;
        memory::set_value(GPIO_RCGC_GPIO_R, 0x20);

        GPIORegister {
            cr_r: (port + 0x524) as *mut u32,
            amsel_r: (port + 0x400) as *mut u32,
            pctl_r: (port + 0x52C) as *mut u32,
            dir_r: (port + 0x400) as *mut u32,
            afsel_r: (port + 0x420) as *mut u32,
            pur_r: (port + 0x510) as *mut u32,
            den_r: (port + 0x51C) as *mut u32,
            data_r: (port + 0x3FC) as *mut u32,
        }
    }
}

pub struct DigitalPin {
    registers: GPIORegister,
    pin: u32,
}

impl DigitalPin {
    pub fn new(port: Port, pin: Pins) -> DigitalPin {
        let registers = GPIORegister::new(port);
        let pin_value = pin as u32;

        memory::set(registers.cr_r, pin_value);
        memory::clear(registers.amsel_r, pin_value);
        memory::clear(registers.pctl_r, pin_value);
        memory::clear(registers.afsel_r, pin_value);
        memory::set(registers.den_r, pin_value);

        DigitalPin{registers: registers, pin: pin_value}
    }

    pub fn make_output(&self) {
        memory::set(self.registers.dir_r, self.pin);
        memory::clear(self.registers.pur_r, self.pin);
    }

    pub fn make_input(&self) {
        memory::clear(self.registers.dir_r, self.pin);
        memory::set(self.registers.pur_r, self.pin);

    }

    pub fn read(&self) -> u32 {
        memory::read(self.registers.data_r, self.pin)
    }

    pub fn set(&self) {
        memory::set(self.registers.data_r, self.pin);
    }

    pub fn clear(&self) {
        memory::clear(self.registers.data_r, self.pin);
    }

}