//! Contains the definition of a response message, used to respond to requests.

use crate::{settings::NodeSettings, version::Version};
use derive_more::Debug;
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

    /// The client made an invalid request.
    InvalidRequest,

    /// Client exceeded the server's rate limit.
    RateLimitExceeded,

    /// An error occurred on the server while processing the request.
    InternalServerError,

    /// Kicked for stalling.
    Stalling,

    /// No new firmware update is available.
    FirmwareUpToDate,

    /// Firmware update available.
    UpdateAvailable(Version),

    /// Part of a firmware update.
    UpdatePart(#[debug(skip)] Box<[u8]>),

    /// End of firmware update chunks.
    UpdateEnd,

    /// Node settings.
    Settings(Option<NodeSettings>),
}
