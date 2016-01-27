use core::str;
use core::str::FromStr;
use hal::uart;

const BUFFER_SIZE: usize = 3;

pub struct Uart {
    device: uart::Device,
    replay_output: bool,
    buffer: [u8; BUFFER_SIZE],
    used_buffer_size: usize,
}

impl Uart {
    pub fn new(device: uart::Uarts, replay_output: bool) -> Uart {
        Uart {
            device: uart::Device::new(device),
            replay_output: replay_output,
            buffer: ['\0' as u8; BUFFER_SIZE],
            used_buffer_size: 0,
        }
    }

    pub fn read_int(&mut self) -> i32 {
        let is_numeric = |c| c >= 48 && c <= 57;
        self.buffer = ['\0' as u8; BUFFER_SIZE];
        self.used_buffer_size = 0;

        self.read(is_numeric);
        let input_str = self.view_buffer_as_str();

        let result = i32::from_str(input_str).unwrap();

        result
    }

    fn read<F>(&mut self, predicate: F)
        where F: Fn(u8) -> bool {
        let mut next_char = self.read_u8();

        while predicate(next_char) {
            self.buffer[self.used_buffer_size] = next_char;
            self.used_buffer_size = self.used_buffer_size + 1;

            if self.used_buffer_size == BUFFER_SIZE {
                break;
            }

            next_char = self.read_u8();
        }
    }

    fn view_buffer_as_str(&self) -> &str {
        str::from_utf8(&self.buffer[0..self.used_buffer_size]).unwrap()
    }

    fn read_u8(&self) -> u8 {
        let value = self.device.read_char();

        if self.replay_output && !self.is_new_line(value) {
            self.put_u8(value);
        }

        value
    }

    fn put_u8(&self, value: u8) {
        if value != 10 && value != 13 {
            self.device.put(value);
        } else {
            self.device.put(13);
            self.device.put(10);
        }
    }

    fn is_new_line(&self, value: u8) -> bool {
        value == 10 || value == 13
    }

}

impl ::core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.put_u8(byte);
        }

        Ok(())
    }
}
