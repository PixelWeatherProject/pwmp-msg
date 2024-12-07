use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeSettings {
    pub battery_ignore: bool,
    pub ota: bool,
    pub sleep_time: u16,
    pub sbop: bool,
    pub mute_notifications: bool,
}
