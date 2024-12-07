use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeSettings {
    battery_ignore: bool,
    ota: bool,
    sleep_time: u16,
    sbop: bool,
    mute_notifications: bool,
}
