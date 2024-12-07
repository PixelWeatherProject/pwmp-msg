use crate::settings::SettingValue;
use serde::{Deserialize, Serialize};

/// A response message used by the PWMP server to respond to [`Request`](crate::request::Request)s.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Response {
    /// A response message to [`Request::Ping`](crate::request::Request::Ping).
    Pong,

    /// Indicate a successfully processed request. This is usually used as a response to `Request::Post*` messages.
    Ok,

    /// The server has rejected authentication. The node/client will be disconnected.
    Reject,

    /// No new firmware update is available.
    FirmwareUpToDate,

    /// Firmware update available.
    UpdateAvailable(u8, u8, u8),

    /// Part of a firmware update.
    UpdatePart(Box<[u8]>),

    /// End of firmware update chunks.
    UpdateEnd,

    /// List of setting values requested by [`Request::GetSettings`](crate::request::Request::GetSettings).
    /// The values are ordered in the same way as they were requested.
    Settings(Box<[SettingValue]>),
}
