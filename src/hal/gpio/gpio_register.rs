use memory;
use super::port_data::PortData;

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
    pub fn new(port: PortData) -> GPIORegister {
        GPIORegister::power_up_port(port.position);

        GPIORegister {
            cr_r: (port.base_address + 0x524) as *mut u32,
            amsel_r: (port.base_address + 0x400) as *mut u32,
            pctl_r: (port.base_address + 0x52C) as *mut u32,
            dir_r: (port.base_address + 0x400) as *mut u32,
            afsel_r: (port.base_address + 0x420) as *mut u32,
            pur_r: (port.base_address + 0x510) as *mut u32,
            den_r: (port.base_address + 0x51C) as *mut u32,
            data_r: (port.base_address + 0x3FC) as *mut u32,
            lock_r: (port.base_address + 0x520) as *mut u32,
        }
    }

    fn power_up_port(position: u32) {
        unsafe {
            memory::set(GPIO_RCGC_GPIO_R, position);
            // dummy read because we have to wait for the clock to stabilize
            memory::read(GPIO_RCGC_GPIO_R, 0x1);
        }
    }
}
