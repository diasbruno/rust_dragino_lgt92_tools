pub mod lgt92;
pub mod decoder;

#[cfg(test)]
mod decode_tests {
    use bytes::Bytes;

    use crate::{lgt92::{Alarm, LGT92}, decoder};

    #[test]
    fn decode() {
	let x: [u8; 12] = [0x06, 0x76, 0x5f, // lat
			   0xF2, 0x96, 0x0A, // long
			   0x4B, 0x45, // alarm & bat
			   0x04, 0xD2, // Roll
			   0xFB, 0x2F as u8]; // pitch

	let b = Bytes::from_iter(x);
	let p: LGT92 = decoder::decode(&b);

	assert_eq!(p.latitude, 42.3519);
	assert_eq!(p.longitude, -87.9094);
	assert_eq!(p.alarm, Alarm::On);
	assert_eq!(p.battery, 2885.0);
	assert_eq!(p.roll, 12.34);
	assert_eq!(p.pitch, -12.33);
    }
}
