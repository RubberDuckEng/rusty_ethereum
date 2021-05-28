use std::fmt;

use std::ops::Add;

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub struct UInt256 {
    high: u128,
    low: u128,
}

impl Add for UInt256 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let carry_result = self.low.overflowing_add(other.low);
        let new_low = carry_result.0;
        let mut new_high = self.high + other.high;
        if carry_result.1 {
            new_high += 1;
        }
        Self {
            low: new_low,
            high: new_high,
        }
    }
}

fn u128_from_be_slice(bytes: &[u8]) -> u128 {
    let mut word: u128 = 0;
    for byte in bytes {
        word <<= 8;
        word += *byte as u128;
    }
    return word;
}

impl UInt256 {
    pub fn from_be_slice(bytes: &[u8]) -> UInt256 {
        if bytes.len() > 8 {
            return UInt256 {
                high: u128_from_be_slice(&bytes[8..]),
                low: u128_from_be_slice(&bytes[..8]),
            };
        }
        return UInt256 {
            high: 0,
            low: u128_from_be_slice(bytes),
        };
    }
}

impl fmt::Debug for UInt256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.high == 0 {
            write!(f, "0x{:02X}", self.low)
        } else {
            write!(f, "0x{:X}{:02X}", self.high, self.low)
        }
    }
}

impl fmt::Display for UInt256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.high == 0 {
            write!(f, "(0x{:02X})", self.low)
        } else {
            write!(f, "(0x{:X}{:02X})", self.high, self.low)
        }
    }
}
