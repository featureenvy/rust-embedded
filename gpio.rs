use memory;

const GPIO_RCGC_GPIO_R: *mut u32 = 0x400FE608 as *mut u32;

pub enum Port {
    PORTF = 0x4002_5000,
}

pub struct GPIO {
    cr_r: *mut u32,
    amsel_r: *mut u32,
    pctl_r: *mut u32,
    dir_r: *mut u32,
    afsel_r: *mut u32,
    pur_r: *mut u32,
    den_r: *mut u32,
    data_r: *mut u32,
}

impl GPIO {
    pub fn new(port_name: Port) -> GPIO {
        let port = port_name as u32;
        memory::set(GPIO_RCGC_GPIO_R, 0x20);

        GPIO {
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

    pub fn make_output(&self) {
        memory::write(self.cr_r, 0x1F);
        memory::write(self.amsel_r, 0x00);
        memory::write(self.pctl_r, 0x0);
        memory::write(self.dir_r, 0x0E);
        memory::write(self.afsel_r, 0x0);
        memory::write(self.pur_r, 0x11);
        memory::write(self.den_r, 0x02);
    }

    fn read(&self, mask: u32) -> u32 {
        memory::read(self.data_r, mask)
    }

    fn write(&self, value: u32) {
        memory::write(self.data_r, value);
    }

}

pub fn read(port_name: Port, mask: u32) -> u32 {

    let port = GPIO::new(port_name);
    port.read(mask)
}

pub fn write(port_name: Port, value: u32) {
    let port = GPIO::new(port_name);
    port.write(value);
}
