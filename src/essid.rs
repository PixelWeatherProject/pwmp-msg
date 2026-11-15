//! Type definitions for a type representing Wi-Fi access point names (ESSID).

use serde::{Deserialize, Serialize};
use std::{fmt::Debug, ops::Deref};

/// Maximum length of an ESSID
const WIFI_ESSID_MAX_LEN: usize = 32;

/// Wi-Fi access point name (ESSID).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WifiEssid(#[serde(with = "serde_bytes")] [u8; WIFI_ESSID_MAX_LEN]);

impl WifiEssid {
    /// Creates a new empty string.
    #[must_use]
    pub const fn const_new() -> Self {
        Self([0; WIFI_ESSID_MAX_LEN])
    }

    /// Returns the length of the string.
    #[must_use]
    pub fn len(&self) -> usize {
        for (index, byte) in self.0.iter().enumerate() {
            if *byte == 0 {
                return index;
            }
        }

        WIFI_ESSID_MAX_LEN
    }

    /// Returns whether the string is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0[0] == 0
    }

    /// Returns the ESSID string as an array of bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..self.len()]
    }
}

impl TryFrom<&str> for WifiEssid {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > WIFI_ESSID_MAX_LEN {
            return Err(());
        }

        let mut buffer = [0; WIFI_ESSID_MAX_LEN];
        buffer[..value.len()].copy_from_slice(value.as_bytes());
        Ok(Self(buffer))
    }
}

impl TryFrom<String> for WifiEssid {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<Box<str>> for WifiEssid {
    type Error = ();

    fn try_from(value: Box<str>) -> Result<Self, Self::Error> {
        value.as_ref().try_into()
    }
}

impl Deref for WifiEssid {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // SAFETY: This is always a valid UTF-8 string.
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }
}

impl Default for WifiEssid {
    fn default() -> Self {
        Self::const_new()
    }
}

impl Debug for WifiEssid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &**self)
    }
}
