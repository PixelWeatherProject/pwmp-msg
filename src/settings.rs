//! Settings type for representing individual node settings.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Settings of a particular node.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeSettings {
    /// Whether to ignore low battery voltage.
    pub battery_ignore: bool,

    /// Whether Over-the-Air (remote) updates should be allowed.
    pub ota: bool,

    /// Amount of time *in seconds* to sleep after every session.
    pub sleep_time: u16,

    /// Whether to enable software-based battery over-discharge protection.
    pub sbop: bool,

    /// Whether the node is allowed to send notifications.
    pub mute_notifications: bool,
}

impl NodeSettings {
    #[must_use]
    /// Returns the sleep time represented using [`Duration`](Duration).
    pub const fn sleep_time(&self) -> Duration {
        Duration::from_secs(self.sleep_time as _)
    }
}

impl Default for NodeSettings {
    fn default() -> Self {
        Self {
            battery_ignore: false,
            ota: true,
            sleep_time: 60,
            sbop: true,
            mute_notifications: false,
        }
    }
}
