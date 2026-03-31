//! Settings type for representing individual node settings.

use std::time::Duration;

/// Settings of a particular node.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    /// Returns the sleep time represented using [`Duration`](Duration).
    ///
    /// ```rust
    /// use pwmp_msg::settings::NodeSettings;
    /// use std::time::Duration;
    ///
    /// let mut settings = NodeSettings::default();
    /// settings.sleep_time = 555;
    ///
    /// assert_eq!(settings.sleep_time(), Duration::from_secs(555));
    /// ```
    #[must_use]
    pub const fn sleep_time(&self) -> Duration {
        Duration::from_secs(self.sleep_time as _)
    }

    /// Create a new instance with default values.
    ///
    /// This is an alternative to [`Default::default()`] that is `const`.
    /// Useful for creating `static`s without the need for `Lazy*` or `Once*`
    /// primitives.
    ///
    /// ```rust
    /// use pwmp_msg::settings::NodeSettings;
    ///
    /// static SETTINGS: NodeSettings = NodeSettings::const_default();
    /// ```
    #[must_use]
    pub const fn const_default() -> Self {
        Self {
            battery_ignore: false,
            ota: true,
            sleep_time: 60,
            sbop: true,
            mute_notifications: false,
        }
    }
}

impl Default for NodeSettings {
    fn default() -> Self {
        Self::const_default()
    }
}
