use bytes::Bytes;
use crate::lgt92::{LGT92, Alarm};

const SCALE: f32 = 10_000.0;

pub fn decode_lat(b: &Bytes) -> f32 {
    let r: u32 = (b[0] as u32) << 16 | (b[1] as u32) << 8 | b[2] as u32;

    if (r & 0x800000) == 0x0 {
        return (r as f32) / SCALE
    }

    r as f32
}

pub fn decode_long(b: &Bytes) -> f32 {
    let s: u32 = (b[3] as u32) << 16 | (b[4] as u32) << 8 | b[5] as u32;

    if (s & 0x800000) == 0x800000 {
        return (s as f32 - 0x1000000 as f32) / SCALE
    }

    s as f32
}

pub fn decode_alarm(b: &Bytes) -> Alarm {
    let a: u8 = b[6];
    if (a & 0x40) == 0x40 {
        return Alarm::On
    }
    Alarm::Off
}

pub fn decode_battery(b: &Bytes) -> f32 {
    let bat = (b[6] as u16) << 8 | b[7] as u16;
    (bat & 0x3FFF) as f32
}

pub fn decode_roll(b: &Bytes) -> f32 {
    let roll = (b[8] as u16) << 8 | b[9] as u16;

    println!("roll {:#x}", roll);

    if (roll & 0x8000) == 0x0 {
        return (roll as f32) / 100.0
    }

    roll as f32
}

pub fn decode_pitch(b: &Bytes) -> f32 {
    let pitch = (b[10] as u16) << 8 | b[11] as u16;

    println!("pitch {:?}", pitch);

    if (pitch & 0x8000) == 0x8000 {
        return ((pitch as f32) - (0x10000 as f32)) / 100.0
    }

    pitch as f32
}

pub fn decode(b: &Bytes) -> LGT92 {
    LGT92 {
        latitude: decode_lat(b),
        longitude: decode_long(b),
        alarm: decode_alarm(b),
        battery: decode_battery(b),
        roll: decode_roll(b),
        pitch: decode_pitch(b)
    }
}
