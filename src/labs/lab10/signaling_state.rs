use super::{SignalColors, PedestrianColors};

pub struct AllRed;
pub struct GoWest;
pub struct LastCallWest;
pub struct GoNorth;
pub struct LastCallNorth;
pub struct GoPedestrian;
pub struct LastCallPedestrian;

pub static ALL_RED_REF: AllRed = AllRed;
pub static GO_WEST_REF: GoWest = GoWest;
pub static LAST_CALL_WEST_REF: LastCallWest = LastCallWest;
pub static GO_NORTH_REF: GoNorth = GoNorth;
pub static LAST_CALL_NORTH_REF: LastCallNorth = LastCallNorth;
pub static GO_PEDESTRIAN_REF: GoPedestrian = GoPedestrian;
pub static LAST_CALL_PEDESTRIAN_REF: LastCallPedestrian = LastCallPedestrian;

pub struct SignalValues {
    pub pedestrian: PedestrianColors,
    pub west: SignalColors,
    pub north: SignalColors,
}

pub trait State {
    fn next(&self, pedestrian: bool, west: bool, north: bool) -> &'static State;
    fn get_signal_values(&self) -> SignalValues;
    fn wait_duration(&self) -> u32;
}

impl State for AllRed {
    fn next(&self, pedestrian: bool, west: bool, north: bool) -> &'static State {
        match (pedestrian, west, north) {
            (true, _, _) => &GO_PEDESTRIAN_REF,
            (_, true, _) => &GO_WEST_REF,
            (_, _, true) => &GO_NORTH_REF,
            (_, _, _) => &ALL_RED_REF
        }
    }

    fn get_signal_values(&self) -> SignalValues {
        SignalValues {
            pedestrian: PedestrianColors::Red,
            west: SignalColors::Red,
            north: SignalColors::Red
        }
    }

    fn wait_duration(&self) -> u32 {
        50
    }
}

impl State for GoWest {
    fn next(&self, _: bool, _: bool, _: bool) -> &'static State {
        &LAST_CALL_WEST_REF
    }

    fn get_signal_values(&self) -> SignalValues {
        SignalValues {
            pedestrian: PedestrianColors::Red,
            west: SignalColors::Green,
            north: SignalColors::Red
        }
    }

    fn wait_duration(&self) -> u32 {
        50
    }
}

impl State for LastCallWest {
    fn next(&self, pedestrian: bool, _: bool, north: bool) -> &'static State {
        match (pedestrian, north) {
            (_, true) => &GO_NORTH_REF,
            (true, _) => &GO_PEDESTRIAN_REF,
            (_, _) => &ALL_RED_REF
        }
    }

    fn get_signal_values(&self) -> SignalValues {
        SignalValues {
            pedestrian: PedestrianColors::Red,
            west: SignalColors::Yellow,
            north: SignalColors::Red
        }
    }

    fn wait_duration(&self) -> u32 {
        50
    }
}

impl State for GoNorth {
    fn next(&self, _: bool, _: bool, _: bool) -> &'static State {
        &LAST_CALL_NORTH_REF
    }

    fn get_signal_values(&self) -> SignalValues {
        SignalValues {
            pedestrian: PedestrianColors::Red,
            west: SignalColors::Red,
            north: SignalColors::Green
        }
    }

    fn wait_duration(&self) -> u32 {
        50
    }
}

impl State for LastCallNorth {
    fn next(&self, pedestrian: bool, west: bool, _: bool) -> &'static State {
        match (pedestrian, west) {
            (true, _) => &GO_PEDESTRIAN_REF,
            (_, true) => &GO_WEST_REF,
            (_, _) => &ALL_RED_REF
        }
    }

    fn get_signal_values(&self) -> SignalValues {
        SignalValues {
            pedestrian: PedestrianColors::Red,
            west: SignalColors::Red,
            north: SignalColors::Yellow
        }
    }

    fn wait_duration(&self) -> u32 {
        50
    }
}

impl State for GoPedestrian {
    fn next(&self, _: bool, _: bool, _: bool) -> &'static State {
        &LAST_CALL_PEDESTRIAN_REF
    }

    fn get_signal_values(&self) -> SignalValues {
        SignalValues {
            pedestrian: PedestrianColors::Green,
            west: SignalColors::Red,
            north: SignalColors::Red
        }
    }

    fn wait_duration(&self) -> u32 {
        50
    }
}

impl State for LastCallPedestrian {
    fn next(&self, _: bool, west: bool, north: bool) -> &'static State {
        match (west, north) {
            (true, _) => &GO_WEST_REF,
            (_, true) => &GO_NORTH_REF,
            (_, _) => &ALL_RED_REF
        }
    }

    fn get_signal_values(&self) -> SignalValues {
        SignalValues {
            pedestrian: PedestrianColors::Off,
            west: SignalColors::Red,
            north: SignalColors::Red
        }
    }

    fn wait_duration(&self) -> u32 {
        50
    }
}
