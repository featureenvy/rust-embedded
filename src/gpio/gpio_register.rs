use memory;
use super::{Port};

const GPIO_RCGC_GPIO_R: *mut u32 = 0x400FE608 as *mut u32;

pub struct GPIORegister {
    pub cr_r: *mut u32,
    pub amsel_r: *mut u32,
    pub pctl_r: *mut u32,
    pub dir_r: *mut u32,
    pub afsel_r: *mut u32,
    pub pur_r: *mut u32,
    pub den_r: *mut u32,
    pub data_r: *mut u32,
    pub lock_r: *mut u32,
}

impl GPIORegister {
    pub fn new(port_name: Port) -> GPIORegister {
        let port = port_name as u32;
        unsafe {
            memory::set(GPIO_RCGC_GPIO_R, 5);
        }

        GPIORegister {
            cr_r: (port + 0x524) as *mut u32,
            amsel_r: (port + 0x400) as *mut u32,
            pctl_r: (port + 0x52C) as *mut u32,
            dir_r: (port + 0x400) as *mut u32,
            afsel_r: (port + 0x420) as *mut u32,
            pur_r: (port + 0x510) as *mut u32,
            den_r: (port + 0x51C) as *mut u32,
            data_r: (port + 0x3FC) as *mut u32,
            lock_r: (port + 0x520) as *mut u32,
        }
    }
}
