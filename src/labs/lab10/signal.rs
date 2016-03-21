use driver::led;

use super::SignalColors;

pub struct Signal {
    wait: led::Led,
    last_call: led::Led,
    go: led::Led,
}

impl Signal {
    pub fn new(wait: led::Led, last_call: led::Led, go: led::Led) -> Signal {
        wait.on();
        last_call.off();
        go.off();

        Signal {
            wait: wait,
            last_call: last_call,
            go: go
        }
    }

    pub fn set(&self, value: SignalColors) {
        match value {
            SignalColors::Red => {
                self.wait.on();
                self.last_call.off();
                self.go.off();
            },
            SignalColors::Yellow => {
                self.wait.off();
                self.last_call.on();
                self.go.off();
            },
            SignalColors::Green => {
                self.wait.off();
                self.last_call.off();
                self.go.on();
            }
        }
    }
}

