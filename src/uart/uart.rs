use super::Uarts;
use super::device::Device;

pub struct Uart {
    device: Device,
}

impl Uart {
    pub fn new(device: Uarts) -> Uart {
        Uart {
            device: Device::new(device)
        }
    }

    pub fn write(&self, value: u32) {
        self.device.put(value);
    }

    pub fn read(&self) -> u32 {
        self.device.read_char()
    }
}
