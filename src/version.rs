use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Version {
    major: u8,
    middle: u8,
    minor: u8,
}

impl Version {
    #[must_use]
    pub const fn new(major: u8, middle: u8, minor: u8) -> Self {
        Self {
            major,
            middle,
            minor,
        }
    }

    pub fn parse<S: AsRef<str>>(input: S) -> Option<Self> {
        let mut parts = input.as_ref().splitn(3, '.');

        let major = u8::from_str(parts.next()?).ok()?;
        let middle = u8::from_str(parts.next()?).ok()?;
        let minor = u8::from_str(parts.next()?).ok()?;

        Some(Self::new(major, middle, minor))
    }

    #[must_use]
    pub const fn major(&self) -> u8 {
        self.major
    }

    #[must_use]
    pub const fn middle(&self) -> u8 {
        self.middle
    }

    #[must_use]
    pub const fn minor(&self) -> u8 {
        self.minor
    }

    #[must_use]
    pub const fn to_signed_triple(&self) -> (i16, i16, i16) {
        (self.major as _, self.middle as _, self.minor as _)
    }
}

impl From<Version> for (u8, u8, u8) {
    fn from(value: Version) -> Self {
        (value.major, value.middle, value.minor)
    }
}

impl From<(u8, u8, u8)> for Version {
    fn from(value: (u8, u8, u8)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.middle, self.minor)
    }
}
