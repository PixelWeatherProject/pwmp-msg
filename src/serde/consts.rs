//! Internal constants for serialization and deserialization.

/// Byte for representing that an optional value is empty.
pub const OPTIONAL_EMPTY: u8 = 0;

/// Byte for representing that an optional value has a value.
pub const OPTIONAL_EXIST: u8 = 1;

/// Byte for representing that an optional value is empty.
pub const MSG_KIND_REQUEST: u8 = 4;

/// Byte for representing that an optional value has a value.
pub const MSG_KIND_RESPONSE: u8 = 6;

// reserve 7~9

// REQUESTS - 10~30

/// Byte for representing the request variant [`Request::Ping`](crate::request::Request::Ping).
pub const REQ_KIND_PING: u8 = 10;

/// Byte for representing the request variant [`Request::Handshake`](crate::request::Request::Handshake).
pub const REQ_KIND_HANDSHAKE: u8 = 11;

/// Byte for representing the request variant [`Request::PostResults`](crate::request::Request::PostResults).
pub const REQ_KIND_POST_RESULTS: u8 = 12;

/// Byte for representing the request variant [`Request::PostStats`](crate::request::Request::PostStats).
pub const REQ_KIND_POST_STATS: u8 = 13;

/// Byte for representing the request variant [`Request::SendNotification`](crate::request::Request::SendNotification).
pub const REQ_KIND_SEND_NOTIFICATION: u8 = 14;

/// Byte for representing the request variant [`Request::GetSettings`](crate::request::Request::GetSettings).
pub const REQ_KIND_GET_SETTINGS: u8 = 15;

/// Byte for representing the request variant [`Request::UpdateCheck`](crate::request::Request::UpdateCheck).
pub const REQ_KIND_UPDATE_CHECK: u8 = 16;

/// Byte for representing the request variant [`Request::NextUpdateChunk`](crate::request::Request::NextUpdateChunk).
pub const REQ_KIND_NEXT_UPDATE_CHUNK: u8 = 17;

/// Byte for representing the request variant [`Request::ReportFirmwareUpdate`](crate::request::Request::ReportFirmwareUpdate).
pub const REQ_KIND_REPORT_FWU: u8 = 18;

/// Byte for representing the request variant [`Request::Bye`](crate::request::Request::Bye).
pub const REQ_KIND_BYE: u8 = 19;

// reserve 20~30

// RESPONSES - 40~60

/// Byte for representing the response variant [`Response::Pong`](crate::response::Response::Pong).
pub const RES_KIND_PONG: u8 = 40;

/// Byte for representing the response variant [`Response::Ok`](crate::response::Response::Ok).
pub const RES_KIND_OK: u8 = 41;

/// Byte for representing the response variant [`Response::Reject`](crate::response::Response::Reject).
pub const RES_KIND_REJECT: u8 = 42;

/// Byte for representing the response variant [`Response::InvalidRequest`](crate::response::Response::InvalidRequest).
pub const RES_KIND_INVALID_REQ: u8 = 43;

/// Byte for representing the response variant [`Response::RateLimitExceeded`](crate::response::Response::RateLimitExceeded).
pub const RES_KIND_RLE: u8 = 44;

/// Byte for representing the response variant [`Response::InternalServerError`](crate::response::Response::InternalServerError).
pub const RES_KIND_ISE: u8 = 45;

/// Byte for representing the response variant [`Response::Stalling`](crate::response::Response::Stalling).
pub const RES_KIND_STALLING: u8 = 46;

/// Byte for representing the response variant [`Response::FirmwareUpToDate`](crate::response::Response::FirmwareUpToDate).
pub const RES_KIND_FW_UTD: u8 = 47;

/// Byte for representing the response variant [`Response::UpdateAvailable`](crate::response::Response::UpdateAvailable).
pub const RES_KIND_FW_UAVAIL: u8 = 48;

/// Byte for representing the response variant [`Response::UpdatePart`](crate::response::Response::UpdatePart).
pub const RES_KIND_FW_UPART: u8 = 49;

/// Byte for representing the response variant [`Response::UpdateEnd`](crate::response::Response::UpdateEnd).
pub const RES_KIND_FW_UEND: u8 = 50;

/// Byte for representing the response variant [`Response::Settings`](crate::response::Response::Settings).
pub const RES_KIND_SETTINGS: u8 = 51;

// reserve 52~60
