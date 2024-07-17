#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Number(u8);
impl Number {
    pub const fn new(num: u8) -> Self {
        if num < 1 || num > 9 {
            panic!("number out of range");
        }
        Self(num)
    }
    pub const fn try_new(num: u8) -> Option<Self> {
        if num < 1 || num > 9 {
            return None;
        }
        Some(Self(num))
    }
    pub const fn number(self) -> u8 {
        self.0
    }
}

// impl Deref for Number {
//     type Target = u8;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

use std::num::ParseIntError;
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseNumberError {
    #[error("number out of range: {0}")]
    OutOfRange(u8),
    #[error(transparent)]
    InvalidNumber(#[from] ParseIntError),
}

impl std::str::FromStr for Number {
    type Err = ParseNumberError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = match s.parse::<u8>() {
            Ok(n) => n,
            Err(e) => return Err(ParseNumberError::InvalidNumber(e)),
        };
        Self::try_new(n).ok_or(ParseNumberError::OutOfRange(n))
    }
}
