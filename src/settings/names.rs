use super::values::SettingValue;
use serde::{Deserialize, Serialize};

/// A node setting name.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum SettingName {
    /// Whether to ignore the battery level.
    BatteryIgnore,
    /// Wheter to allow OTA firmware updates.
    Ota,
    /// Time to sleep for after posing measurements.
    SleepTime,
    /// Software-based battery overdischarge protection.
    Sbop,
    /// Wheter to mute all notifications.
    MuteNotifications,
}

impl SettingName {
    /// Convert the setting it's string representation.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::BatteryIgnore => "battery_ignore",
            Self::Ota => "ota",
            Self::SleepTime => "sleep_time",
            Self::Sbop => "sbop",
            Self::MuteNotifications => "mute_notifications",
        }
    }

    /// Returns the default value for the setting.
    #[must_use]
    pub fn default_value(self) -> SettingValue {
        match self {
            Self::BatteryIgnore | Self::Ota | Self::MuteNotifications => false.into(),
            Self::SleepTime => 60.into(),
            Self::Sbop => true.into(),
        }
    }
}
