use hal::{clock, gpio};
use driver::{led, switch};

enum SignalColors {
    Green,
    Yellow,
    Red,
}

struct Signal {
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

struct SignalValues {
    pedestrian: SignalColors,
    west: SignalColors,
    north: SignalColors,
}

enum SignalingState {
    AllRed,
    GoWest,
    LastCallWest,
}

struct SignalingSystem {
    state: SignalingState,
}

impl SignalingSystem {
    pub fn new() -> SignalingSystem {
        SignalingSystem {
            state: SignalingState::AllRed,
        }
    }

    pub fn next(&mut self, pedestrian: bool, west: bool, north: bool) {
        match self.state {
            SignalingState::AllRed => {
                match (pedestrian, west, north) {
                    (_, true, _) => self.state = SignalingState::GoWest,
                    (_, false, _) => self.state = SignalingState::AllRed
                }
            },
            SignalingState::GoWest => {
                self.state = SignalingState::LastCallWest
            },
            SignalingState::LastCallWest => {
                self.state = SignalingState::AllRed
            }
        }
    }

    pub fn get_signal_values(&self) -> SignalValues {
        match self.state {
            SignalingState::AllRed => {
                SignalValues {
                    pedestrian: SignalColors::Red,
                    west: SignalColors::Red,
                    north: SignalColors::Red
                }
            },
            SignalingState::GoWest => {
                SignalValues {
                    pedestrian: SignalColors::Red,
                    west: SignalColors::Green,
                    north: SignalColors::Red
                }
            },
            SignalingState::LastCallWest => {
                SignalValues {
                    pedestrian: SignalColors::Red,
                    west: SignalColors::Yellow,
                    north: SignalColors::Red
                }
            }
        }
    }

    pub fn wait_duration(&self) -> u64 {
        50
    }
}

pub fn run() {
    clock::init();

    // http://stackoverflow.com/questions/35439546/a-pattern-for-finite-game-state-machine-in-rust-with-changing-behavior

    let pedestrian_red = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);
    let pedestrian_green = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin0);
    let pedestrian_waiting = switch::Switch::new(gpio::Port::PortE, gpio::Pins::Pin2, gpio::Logic::Negative);

    let pedestrian_signal = Signal::new(pedestrian_red, pedestrian_red, pedestrian_green);

    let west_red = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin5);
    let west_yellow = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin4);
    let west_green = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin3);
    let west_waiting = switch::Switch::new(gpio::Port::PortE, gpio::Pins::Pin0, gpio::Logic::Negative);

    let west_signal = Signal::new(west_red, west_yellow, west_green);

    let north_red = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin2);
    let north_yellow = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin1);
    let north_green = led::Led::new(gpio::Port::PortB, gpio::Pins::Pin0);
    let north_waiting = switch::Switch::new(gpio::Port::PortE, gpio::Pins::Pin1, gpio::Logic::Negative);

    let north_signal = Signal::new(north_red, north_yellow, north_green);

    let mut signaling_system = SignalingSystem::new();

    loop {
        let signal_values = signaling_system.get_signal_values();

        pedestrian_signal.set(signal_values.pedestrian);
        west_signal.set(signal_values.west);
        north_signal.set(signal_values.north);

        clock::delay(signaling_system.wait_duration());

        signaling_system.next(pedestrian_waiting.is_on(), west_waiting.is_on(), north_waiting.is_on());
    }
}
