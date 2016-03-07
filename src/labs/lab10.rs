use hal::{clock, gpio};
use driver::{led, switch};

enum SignalColors {
    Green,
    Yellow,
    Red,
}

enum PedestrianColors {
    Red,
    Green,
    Off
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

struct PedestrianSignal {
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


struct SignalValues {
    pedestrian: PedestrianColors,
    west: SignalColors,
    north: SignalColors,
}

mod signaling_state {
    pub struct AllRed;
    pub struct GoWest;
    pub struct LastCallWest;

    pub static ALL_RED_REF: AllRed = AllRed;
    pub static GO_WEST_REF: GoWest = GoWest;
    pub static LAST_CALL_WEST_REF: LastCallWest = LastCallWest;
}

trait State {
    fn next(&self, pedestrian: bool, west: bool, north: bool) -> &'static State;
    fn get_signal_values(&self) -> SignalValues;
    fn wait_duration(&self) -> u64;
}

impl State for signaling_state::AllRed {
    fn next(&self, _: bool, west: bool, _: bool) -> &'static State {
        match west {
            true => &signaling_state::GO_WEST_REF,
            false => &signaling_state::ALL_RED_REF
        }
    }

    fn get_signal_values(&self) -> SignalValues {
        SignalValues {
            pedestrian: PedestrianColors::Red,
            west: SignalColors::Red,
            north: SignalColors::Red
        }
    }

    fn wait_duration(&self) -> u64 {
        50
    }
}

impl State for signaling_state::GoWest {
    fn next(&self, _: bool, _: bool, _: bool) -> &'static State {
        &signaling_state::LAST_CALL_WEST_REF
    }

    fn get_signal_values(&self) -> SignalValues {
        SignalValues {
            pedestrian: PedestrianColors::Red,
            west: SignalColors::Green,
            north: SignalColors::Red
        }
    }

    fn wait_duration(&self) -> u64 {
        50
    }
}

impl State for signaling_state::LastCallWest {
    fn next(&self, _: bool, _: bool, _: bool) -> &'static State {
        &signaling_state::ALL_RED_REF
    }

    fn get_signal_values(&self) -> SignalValues {
        SignalValues {
            pedestrian: PedestrianColors::Red,
            west: SignalColors::Yellow,
            north: SignalColors::Red
        }
    }

    fn wait_duration(&self) -> u64 {
        50
    }
}

pub fn run() {
    clock::init();

    let pedestrian_red = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin1);
    let pedestrian_green = led::Led::new(gpio::Port::PortF, gpio::Pins::Pin0);
    let pedestrian_waiting = switch::Switch::new(gpio::Port::PortE, gpio::Pins::Pin2, gpio::Logic::Negative);

    let pedestrian_signal = PedestrianSignal::new(pedestrian_red, pedestrian_green);

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

    // let mut signaling_system = SignalingSystem::new();
    let mut signaling_system: &'static State = &signaling_state::ALL_RED_REF;

    loop {
        let signal_values = signaling_system.get_signal_values();

        pedestrian_signal.set(signal_values.pedestrian);
        west_signal.set(signal_values.west);
        north_signal.set(signal_values.north);

        clock::delay(signaling_system.wait_duration());

        signaling_system = signaling_system.next(pedestrian_waiting.is_on(), west_waiting.is_on(), north_waiting.is_on());
    }
}
