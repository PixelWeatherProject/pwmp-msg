//! Wi-Fi stats implementation.

use crate::{aliases::Rssi, essid::WifiEssid};
use serde::{Deserialize, Serialize};

/// An object holding Wi-Fi statistical data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WiFiStats {
    /// ESSID of the wireless network
    wifi_ssid: WifiEssid,

    /// RSSI *(signal quality)* of the connection to the wireless network in dBm *(decibel meters)*.
    /// Higher values (closer to 0) mean a better quality.
    wifi_rssi: Rssi,
}

impl WiFiStats {
    /// Creates a new stats object from the specified string and RSSI data.
    ///
    /// If the ESSID is longer than 32 characters (standard limit), then `None` is returned.
    pub fn new<S: AsRef<str>>(essid: S, rssi: Rssi) -> Option<Self> {
        Some(Self {
            wifi_ssid: WifiEssid::try_from(essid.as_ref()).ok()?,
            wifi_rssi: rssi,
        })
    }

    /// Returns the ESSID as a string slice.
    #[must_use]
    pub fn ssid(&self) -> &str {
        &self.wifi_ssid
    }

    /// Returns the signal strength.
    #[must_use]
    pub const fn rssi(&self) -> Rssi {
        self.wifi_rssi
    }
}
