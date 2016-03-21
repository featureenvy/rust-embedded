use driver::led;

use super::PedestrianColors;

pub struct PedestrianSignal {
    wait: led::Led,
    go: led::Led,
}

impl PedestrianSignal {
    pub fn new(wait: led::Led, go: led::Led) -> PedestrianSignal {
        wait.on();
        go.off();

        PedestrianSignal {
            wait: wait,
            go: go
        }
    }

    pub fn set(&self, value: PedestrianColors) {
        match value {
            PedestrianColors::Red => {
                self.wait.on();
                self.go.off();
            },
            PedestrianColors::Green => {
                self.wait.off();
                self.go.on();
            },
            PedestrianColors::Off => {
                self.wait.off();
                self.go.off();
            }
        }
    }
}
