use hal::uart;

pub struct Uart {
    device: uart::Device,
}

impl Uart {
    pub fn new(device: uart::Uarts) -> Uart {
        Uart {
            device: uart::Device::new(device)
        }
    }

    pub fn write(&self, value: u32) {
        self.device.put(value);
    }

    pub fn read(&self) -> u32 {
        self.device.read_char()
    }
}
