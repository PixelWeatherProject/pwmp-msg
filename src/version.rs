use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

/// A structure that represents a semantic version (eg. `1.0.0`) with a major part (`1`), middle part (`0`) and a minor part (`0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct Version {
    /// Major number.
    major: u8,

    /// Middle number.
    middle: u8,

    /// Minor number.
    minor: u8,
}

impl Version {
    /// Create a new instance with the specified parts.
    ///
    /// ```rust
    /// use pwmp_msg::version::Version;
    ///
    /// let version = Version::new(1, 2, 6);
    ///
    /// assert_eq!(version.major(), 1);
    /// assert_eq!(version.middle(), 2);
    /// assert_eq!(version.minor(), 6);
    /// assert_eq!(version.to_string(), "1.2.6");
    /// ```
    #[must_use]
    pub const fn new(major: u8, middle: u8, minor: u8) -> Self {
        Self {
            major,
            middle,
            minor,
        }
    }

    /// Parse a version string.
    ///
    /// ```rust
    /// use pwmp_msg::version::Version;
    ///
    /// let v_string = "1.7.4";
    /// let version = Version::parse(v_string).unwrap();
    /// assert_eq!(version.major(), 1);
    /// assert_eq!(version.middle(), 7);
    /// assert_eq!(version.minor(), 4);
    /// assert_eq!(version.to_string(), v_string);
    ///
    /// // This is not a valid version string.
    /// assert_eq!(Version::parse("1.20.4a"), None);
    /// ```
    pub fn parse<S: AsRef<str>>(input: S) -> Option<Self> {
        let mut parts = input.as_ref().splitn(3, '.');

        let major = u8::from_str(parts.next()?).ok()?;
        let middle = u8::from_str(parts.next()?).ok()?;
        let minor = u8::from_str(parts.next()?).ok()?;

        Some(Self::new(major, middle, minor))
    }

    /// Returns the major number from this version.
    ///
    /// ```rust
    ///  use pwmp_msg::version::Version;
    ///
    /// let from_string = Version::parse("1.2.6").unwrap();
    /// let from_ints = Version::new(1, 2, 6);
    ///
    /// assert_eq!(from_string.major(), from_ints.major());
    /// ```
    #[must_use]
    pub const fn major(&self) -> u8 {
        self.major
    }

    /// Returns the middle number from this version.
    ///
    /// ```rust
    ///  use pwmp_msg::version::Version;
    ///
    /// let from_string = Version::parse("1.2.6").unwrap();
    /// let from_ints = Version::new(1, 2, 6);
    ///
    /// assert_eq!(from_string.middle(), from_ints.middle());
    /// ```
    #[must_use]
    pub const fn middle(&self) -> u8 {
        self.middle
    }

    /// Returns the minor number from this version.
    ///
    /// ```rust
    ///  use pwmp_msg::version::Version;
    ///
    /// let from_string = Version::parse("1.2.6").unwrap();
    /// let from_ints = Version::new(1, 2, 6);
    ///
    /// assert_eq!(from_string.minor(), from_ints.minor());
    /// ```
    #[must_use]
    pub const fn minor(&self) -> u8 {
        self.minor
    }

    /// Converts the version to a triple of signed 16-bit integers.
    ///
    /// ```rust
    ///  use pwmp_msg::version::Version;
    ///
    /// let version = Version::new(1, 4, 7);
    /// let (major, middle, minor): (i16, i16, i16) = version.to_signed_triple();
    ///
    /// assert_eq!(version.major().try_into(), Ok(major));
    /// assert_eq!(version.middle().try_into(), Ok(middle));
    /// assert_eq!(version.minor().try_into(), Ok(minor));
    /// ```
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
