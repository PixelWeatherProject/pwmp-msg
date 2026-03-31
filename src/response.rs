//! Contains the definition of a response message, used to respond to requests.

use crate::{settings::NodeSettings, version::Version};
use derive_more::Debug;

/// A response message used by the PWMP server to respond to [`Request`](crate::request::Request)s.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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

impl Response {
    /// Returns whether the result is of an erroneous variant.
    ///
    /// ```rust
    /// use pwmp_msg::response::Response;
    ///
    /// assert!(!Response::Ok.is_error());
    /// assert!(Response::InternalServerError.is_error());
    /// ```
    #[must_use]
    pub const fn is_error(&self) -> bool {
        matches!(
            self,
            Self::Reject
                | Self::InvalidRequest
                | Self::RateLimitExceeded
                | Self::InternalServerError
                | Self::Stalling
        )
    }
}

impl Response {
    /// Message ID for [`Response::Pong`].
    pub const MSG_ID_PONG: u8 = 0;
    /// Message ID for [`Response::Ok`].
    pub const MSG_ID_OK: u8 = 1;
    /// Message ID for [`Response::Reject`].
    pub const MSG_ID_REJECT: u8 = 2;
    /// Message ID for [`Response::InvalidRequest`].
    pub const MSG_ID_INVALID_REQUEST: u8 = 3;
    /// Message ID for [`Response::RateLimitExceeded`].
    pub const MSG_ID_RATE_LIMIT_EXCEEDED: u8 = 4;
    /// Message ID for [`Response::InternalServerError`].
    pub const MSG_ID_INTERNAL_SERVER_ERROR: u8 = 5;
    /// Message ID for [`Response::Stalling`].
    pub const MSG_ID_STALLING: u8 = 6;
    /// Message ID for [`Response::FirmwareUpToDate`].
    pub const MSG_ID_FIRMWARE_UP_TO_DATE: u8 = 7;
    /// Message ID for [`Response::UpdateAvailable`].
    pub const MSG_ID_UPDATE_AVAILABLE: u8 = 8;
    /// Message ID for [`Response::UpdatePart`].
    pub const MSG_ID_UPDATE_PART: u8 = 9;
    /// Message ID for [`Response::UpdateEnd`].
    pub const MSG_ID_UPDATE_END: u8 = 10;
    /// Message ID for [`Response::Settings`].
    pub const MSG_ID_SETTINGS: u8 = 11;
}
