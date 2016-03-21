mod signal;
mod pedestrian_signal;
mod signaling_state;
mod lab10;

pub enum SignalColors {
    Green,
    Yellow,
    Red,
}

pub enum PedestrianColors {
    Red,
    Green,
    Off
}

pub use self::lab10::run;
