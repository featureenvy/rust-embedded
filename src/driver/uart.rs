use hal::uart;

pub struct Uart {
    device: uart::Device,
    replay_output: bool,
}

impl Uart {
    pub fn new(device: uart::Uarts, replay_output: bool) -> Uart {
        Uart {
            device: uart::Device::new(device),
            replay_output: replay_output,
        }
    }

    pub fn read(&self) -> [u8; 3] {
        let mut output = ['\0' as u8; 3];
        let mut position = 0;

        let mut next_char = self.read_char();

        while next_char != 13 || next_char != 10 {
            output[position] = next_char;
            next_char = self.read_char();

            position = position + 1;

            if position == 3 {
                self.write("Max 3 chars allowed");
                break;
            }
        }

        output

    }

    pub fn write(&self, value: &str) {
        for byte in value.as_bytes() {
            self.put_char(*byte);
        }
    }

    pub fn read_char(&self) -> u8 {
        let value = self.device.read_char();
        if self.replay_output {
            self.put_char(value);
        }

        value
    }

    fn put_char(&self, value: u8) {
        if value != 10 && value != 13 {
            self.device.put(value);
        } else {
            self.device.put(13);
            self.device.put(10);
        }
    }

}
