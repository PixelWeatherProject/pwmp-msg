pub use crate::kick::KickReason;
use crate::{settings::NodeSettings, version::Version};
use serde::{Deserialize, Serialize};

/// A response message used by the PWMP server to respond to [`Request`](crate::request::Request)s.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Response {
    /// A response message to [`Request::Ping`](crate::request::Request::Ping).
    Pong,

    /// Response to a successful handshake, with the node's ID.
    HandshakeSuccess { node_id: i16 },

    /// The server has rejected the handshake. Optionally a reason may be provided.
    HandshakeFailed(Option<Box<str>>),

    /// Indicate a successfully processed request. This is usually used as a response to `Request::Post*` messages.
    Ok,

    /// The server has kicked the client.
    Kicked(KickReason),

    /// No new firmware update is available.
    FirmwareUpToDate,

    /// Firmware update available.
    UpdateAvailable(Version),

    /// Part of a firmware update.
    UpdatePart(Box<[u8]>),

    /// End of firmware update chunks.
    UpdateEnd,

    /// Node settings.
    Settings(Option<NodeSettings>),
}
