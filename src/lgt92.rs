#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Alarm {
    Off = 0,
    On = 1
}

#[derive(Debug, Clone, Copy)]
pub struct LGT92 {
    pub latitude: f32,
    pub longitude: f32,
    pub alarm: Alarm,
    pub battery: f32,
    pub roll: f32,
    pub pitch: f32
}

impl From<u8> for Alarm {
    fn from(switch : u8) -> Self {
	match switch {
	    0 => { Alarm::Off }
	    1..=u8::MAX => { Alarm::On }
	}
    }
}

impl Into<u8> for Alarm {
    fn into(self: Self) -> u8 {
	match self {
	    Alarm::Off => { 0 }
	    Alarm::On => { 1 }
	}
    }
}
