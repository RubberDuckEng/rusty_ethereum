use itertools::Itertools;
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::iter::Iterator;
use std::ops::{Add, Not, Shl, Shr, Sub};

#[derive(Default, Copy, Clone, PartialEq, PartialOrd, Eq)]
pub struct UInt256 {
    high: u128,
    low: u128,
}

pub fn hex_string_as_vec_u8(hex: &str) -> Vec<u8> {
    // NOTE: This does allow 0x0x which might be bad?
    if hex.starts_with("0x") {
        return hex_string_as_vec_u8(&hex[2..]);
    }
    let chars = hex.chars();
    let chunks = chars.chunks(2);
    chunks
        .into_iter()
        .map(|chunk| u8::from_str_radix(&chunk.collect::<String>(), 16).expect("vaild hex"))
        .collect::<Vec<u8>>()
}

impl UInt256 {
    pub const ONE: UInt256 = UInt256 { high: 0, low: 1 };
    pub const ZERO: UInt256 = UInt256 { high: 0, low: 0 };

    pub fn from_bool(value: bool) -> UInt256 {
        if value {
            UInt256::ONE
        } else {
            UInt256::ZERO
        }
    }

    pub fn from_u128(value: u128) -> UInt256 {
        UInt256 {
            low: value,
            high: 0,
        }
    }

    pub fn from_string(hex: &str) -> UInt256 {
        UInt256::from_be_slice(&hex_string_as_vec_u8(hex))
    }
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

impl Sub for UInt256 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        assert_eq!(self.high, 0);
        assert_eq!(other.high, 0);
        println!("Sub for UInt256 is a hack!");
        Self {
            low: self.low.checked_sub(other.low).expect("works"),
            high: 0,
        }
    }
}

impl Not for UInt256 {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            low: !self.low,
            high: !self.high,
        }
    }
}

impl Shr for UInt256 {
    type Output = Self;

    fn shr(self, shift: Self) -> Self {
        // assert not shifting more than 256 (could return 0x0 instead)
        assert_eq!(shift.high, 0);
        assert!(shift.low < 256);
        if shift.low > 128 {
            println!("{} >> {}", self.high, shift.low - 128);
            UInt256 {
                high: 0,
                low: self.high >> (shift.low - 128),
            }
        } else {
            // This case is more complicated:
            // Figure out how many bits apply to high vs. low
            // Mask off any bits needed from high into low.
            // Shift-right high-half.
            // Rotate_right low-half.
            panic!("Not implemented");
        }
    }
}

impl Shl for UInt256 {
    type Output = Self;

    fn shl(self, shift: Self) -> Self {
        // assert not shifting more than 256
        assert_eq!(shift.high, 0);
        assert!(shift.low < 256);
        if shift.low > 128 {
            UInt256 {
                low: 0,
                high: self.low << (shift.low - 128),
            }
        } else {
            // This case is more complicated.
            panic!("Not implemented");
        }
    }
}

impl TryFrom<UInt256> for usize {
    type Error = std::num::TryFromIntError;

    fn try_from(value: UInt256) -> Result<Self, Self::Error> {
        if value.high != 0 {
            // TryFromIntError is not directly construtable?
            // https://stackoverflow.com/questions/54374979/tryfrominterror-usage
            u128::MAX.try_into()
        } else {
            Self::try_from(value.low)
        }
    }
}

impl TryFrom<usize> for UInt256 {
    type Error = std::num::TryFromIntError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let low: u128 = u128::try_from(value)?;
        Ok(UInt256 { low: low, high: 0 })
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
        if bytes.len() > 16 {
            return UInt256 {
                high: u128_from_be_slice(&bytes[..16]),
                low: u128_from_be_slice(&bytes[16..]),
            };
        }
        return UInt256 {
            high: 0,
            low: u128_from_be_slice(bytes),
        };
    }

    pub fn to_be_bytes(&self, bytes: &mut [u8]) {
        bytes[..16].copy_from_slice(&self.high.to_be_bytes());
        bytes[16..].copy_from_slice(&self.low.to_be_bytes());
    }
}

impl fmt::Debug for UInt256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for UInt256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.high == 0 {
            write!(f, "0x{:02X}", self.low)
        } else {
            write!(f, "0x{:X}{:032X}", self.high, self.low)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UInt256;
    #[test]
    fn ordering_works() {
        assert!(UInt256::ONE > UInt256::ZERO);
        assert!(UInt256::ZERO < UInt256::ONE);

        let bigger = UInt256 { high: 2, low: 1 };
        let smaller = UInt256 { high: 1, low: 2 };
        assert!(bigger > smaller);
    }
    #[test]
    fn from_string_works() {
        let value = UInt256::from_string(
            "6D4CE63C00000000000000000000000000000000000000000000000000000000",
        );
        let expected_high: u128 = 0x6D4CE63C << 96;
        let expected_low = 0x00000000;
        assert_eq!(value.low, expected_low);
        assert_eq!(value.high, expected_high);
    }
    #[test]
    fn shr_works() {
        let value = UInt256 {
            high: 0x6D4CE63C << 96,
            low: 0,
        };
        let shift = UInt256::from_u128(0xE0);
        let expected = 0x6D4CE63C;
        assert_eq!(value >> shift, UInt256::from_u128(expected));
    }
    #[test]
    fn not_works() {
        let value = UInt256 { high: 1, low: 0 };
        let expected = UInt256 {
            high: u128::MAX - 1,
            low: u128::MAX,
        };
        assert_eq!(!value, expected);
    }
}
