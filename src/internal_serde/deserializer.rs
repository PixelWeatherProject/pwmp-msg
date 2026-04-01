//! Deserialization utilities.

use super::{BytesLength, OPTIONAL_VALUE_EXISTS, OPTIONAL_VALUE_VACANT};
use crate::{
    mac::Mac, request::Request, response::Response, settings::NodeSettings, version::Version,
    Message,
};
use std::io::{Cursor, Read};
use thiserror::Error;

/// A deserialization error.
#[derive(Debug, Error)]
pub enum DeserializeError {
    /// Empty bytes buffer was provided
    #[error("Empty bytes buffer cannot be deserialized")]
    EmptyBuf,

    /// Not enough bytes could be read from the buffer
    #[error("Buffer was exhausted while reading, expected {0} bytes")]
    Exhausted(usize),

    /// There are still some bytes left in the buffer after deserialization
    #[error("Unprocessed bytes left in the buffer")]
    NotExhausted,

    /// Invalid message type identifier
    #[error("Invalid message type: {0}")]
    IllegalVariantIdentifier(u8),

    /// Invalid [`Request`] type identifier
    #[error("Invalid request message type: {0}")]
    IllegalRequestVariantIdentifier(u8),

    /// Invalid [`Response`] type identifier
    #[error("Invalid response message type: {0}")]
    IllegalResponseVariantIdentifier(u8),

    /// Invalid optional value identifier
    #[error("Optional value identifier '{0}' is invalid")]
    IllegalOptionalIdentifier(u8),

    /// Unable to parse string as UTF-8
    #[error("String is not UTF-8 encoded: {0}")]
    StringDecode(#[from] std::string::FromUtf8Error),

    /// Invalid boolean value (non-0, non-1)
    #[error("Expected boolean value (0 or 1), got {0}")]
    IllegalBooleanValue(u8),

    /// Cursor read operation failed
    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),
}

/// Deserializes a raw byte buffer into a [`Message`].
///
/// # Errors
/// Returns [`DeserializeError`] when:
/// - the input is empty ([`EmptyBuf`](DeserializeError));
/// - the buffer ends before all required fields are read ([`Exhausted`](DeserializeError::Exhausted));
/// - unknown message/request/response identifiers are encountered;
/// - malformed optional/boolean/string fields are found;
/// - extra trailing bytes remain after decoding ([`NotExhausted`](DeserializeError::NotExhausted)).
pub fn deserialize(bytes: &[u8]) -> Result<Message, DeserializeError> {
    if bytes.is_empty() {
        return Err(DeserializeError::EmptyBuf);
    }

    let mut buffer = Cursor::new(bytes);
    let mid = deserialize_byte(&mut buffer)?;
    let kind = deserialize_byte(&mut buffer)?;

    let message = match kind {
        Message::MSG_ID_REQUEST => {
            let req = deserialize_request(&mut buffer)?;
            Message::new_request(req, mid)
        }
        Message::MSG_ID_RESPONSE => {
            let res = deserialize_response(&mut buffer)?;
            Message::new_response(res, mid)
        }
        _ => return Err(DeserializeError::IllegalVariantIdentifier(kind)),
    };

    if buffer.position() != bytes.len() as _ {
        return Err(DeserializeError::NotExhausted);
    }

    Ok(message)
}

/// Deserializes a [`Request`].
///
/// # Errors
/// Read `Errors` section of [`deserialize()`].
fn deserialize_request(buffer: &mut Cursor<&[u8]>) -> Result<Request, DeserializeError> {
    let variant = deserialize_byte(buffer)?;

    match variant {
        Request::MSG_ID_PING => Ok(Request::Ping),
        Request::MSG_ID_HANDSHAKE => {
            let mut mac = Mac::new(0, 0, 0, 0, 0, 0);

            for i in 0..6 {
                mac[i] = deserialize_byte(buffer)?;
            }

            Ok(Request::Handshake { mac })
        }
        Request::MSG_ID_POST_RESULTS => {
            let temperature = deserialize_f32(buffer)?;
            let humidity = deserialize_byte(buffer)?;
            let air_pressure = match deserialize_byte(buffer)? {
                OPTIONAL_VALUE_VACANT => None,
                OPTIONAL_VALUE_EXISTS => Some(deserialize_u16(buffer)?),
                other => return Err(DeserializeError::IllegalOptionalIdentifier(other)),
            };

            Ok(Request::PostResults {
                temperature,
                humidity,
                air_pressure,
            })
        }
        Request::MSG_ID_POST_STATS => {
            let battery = deserialize_f32(buffer)?;
            let wifi_ssid = deserialize_string(buffer)?;
            let wifi_rssi = deserrialize_i8(buffer)?;

            Ok(Request::PostStats {
                battery,
                wifi_ssid,
                wifi_rssi,
            })
        }
        Request::MSG_ID_SEND_NOTIFICATION => {
            Ok(Request::SendNotification(deserialize_string(buffer)?))
        }
        Request::MSG_ID_GET_SETTINGS => Ok(Request::GetSettings),
        Request::MSG_ID_UPDATE_CHECK => Ok(Request::UpdateCheck(deserialize_version(buffer)?)),
        Request::MSG_ID_NEXT_UPDATE_CHUNK => Ok(Request::NextUpdateChunk(deserialize_u32(buffer)?)),
        Request::MSG_ID_REPORT_FIRMWARE_UPDATE => {
            let success = deserialize_bool(buffer)?;
            Ok(Request::ReportFirmwareUpdate(success))
        }
        Request::MSG_ID_BYE => Ok(Request::Bye),
        _ => Err(DeserializeError::IllegalRequestVariantIdentifier(variant)),
    }
}

/// Deserializes a [`Response`].
///
/// # Errors
/// Read `Errors` section of [`deserialize()`].
fn deserialize_response(buffer: &mut Cursor<&[u8]>) -> Result<Response, DeserializeError> {
    let variant = deserialize_byte(buffer)?;

    match variant {
        Response::MSG_ID_PONG => Ok(Response::Pong),
        Response::MSG_ID_OK => Ok(Response::Ok),
        Response::MSG_ID_REJECT => Ok(Response::Reject),
        Response::MSG_ID_INVALID_REQUEST => Ok(Response::InvalidRequest),
        Response::MSG_ID_RATE_LIMIT_EXCEEDED => Ok(Response::RateLimitExceeded),
        Response::MSG_ID_INTERNAL_SERVER_ERROR => Ok(Response::InternalServerError),
        Response::MSG_ID_STALLING => Ok(Response::Stalling),
        Response::MSG_ID_FIRMWARE_UP_TO_DATE => Ok(Response::FirmwareUpToDate),
        Response::MSG_ID_UPDATE_AVAILABLE => {
            Ok(Response::UpdateAvailable(deserialize_version(buffer)?))
        }
        Response::MSG_ID_UPDATE_PART => {
            let blob = deserialize_bytes(buffer)?;
            Ok(Response::UpdatePart(blob))
        }
        Response::MSG_ID_UPDATE_END => Ok(Response::UpdateEnd),
        Response::MSG_ID_SETTINGS => match deserialize_byte(buffer)? {
            OPTIONAL_VALUE_VACANT => Ok(Response::Settings(None)),
            OPTIONAL_VALUE_EXISTS => {
                let battery_ignore = deserialize_bool(buffer)?;
                let ota = deserialize_bool(buffer)?;
                let sleep_time = deserialize_u16(buffer)?;
                let sbop = deserialize_bool(buffer)?;
                let mute_notifications = deserialize_bool(buffer)?;

                Ok(Response::Settings(Some(NodeSettings {
                    battery_ignore,
                    ota,
                    sleep_time,
                    sbop,
                    mute_notifications,
                })))
            }
            other => Err(DeserializeError::IllegalOptionalIdentifier(other)),
        },
        _ => Err(DeserializeError::IllegalResponseVariantIdentifier(variant)),
    }
}

/// Deserializes a [`Version`] payload.
///
/// # Errors
/// Returns an [`Exhausted`](DeserializeError::Exhausted) error if not enough bytes (3)
/// could be read from the cursor.
fn deserialize_version(buffer: &mut Cursor<&[u8]>) -> Result<Version, DeserializeError> {
    let parts: [u8; 3] = read_n_bytes(buffer)?;
    Ok(Version::new(parts[0], parts[1], parts[2]))
}

/// Deserializes a blob payload.
///
/// # Errors
/// Returns an [`Exhausted`](DeserializeError::Exhausted) error if not enough bytes (3)
/// could be read from the cursor, or an [`io::Error`](std::io::Error).
fn deserialize_bytes(buffer: &mut Cursor<&[u8]>) -> Result<Box<[u8]>, DeserializeError> {
    let size: BytesLength = deserialize_u16(buffer)?;
    let mut content = vec![0; usize::from(size)];
    buffer.read_exact(&mut content)?;

    Ok(content.into_boxed_slice())
}

/// Deserializes a UTF-8 encoded string payload.
///
/// # Errors
/// See `Errors` section of [`deserialize_bytes`](deserialize_bytes). Could also return
/// an [`FromUtf8Error`](std::string::FromUtf8Error) if the string is not correctly encoded.
fn deserialize_string(buffer: &mut Cursor<&[u8]>) -> Result<Box<str>, DeserializeError> {
    let bytes = deserialize_bytes(buffer)?;
    Ok(String::from_utf8(bytes.to_vec())?.into_boxed_str())
}

/// Deserializes a boolean payload.
///
/// # Errors
/// Returns an [`IllegalBooleanValue`](DeserializeError::IllegalBooleanValue) error if
/// the byte representing the boolean value is not 0 or 1.
fn deserialize_bool(buffer: &mut Cursor<&[u8]>) -> Result<bool, DeserializeError> {
    match deserialize_byte(buffer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(DeserializeError::IllegalBooleanValue(other)),
    }
}

/// Deserializes an [`f32`] payload.
///
/// # Errors
/// Read `Errors` section of [`read_n_bytes()`].
fn deserialize_f32(buffer: &mut Cursor<&[u8]>) -> Result<f32, DeserializeError> {
    Ok(f32::from_be_bytes(read_n_bytes(buffer)?))
}

/// Deserializes an [`u32`] payload.
///
/// # Errors
/// Read `Errors` section of [`read_n_bytes()`].
fn deserialize_u32(buffer: &mut Cursor<&[u8]>) -> Result<u32, DeserializeError> {
    Ok(u32::from_be_bytes(read_n_bytes(buffer)?))
}

/// Deserializes an [`u16`] payload.
///
/// # Errors
/// Read `Errors` section of [`read_n_bytes()`].
fn deserialize_u16(buffer: &mut Cursor<&[u8]>) -> Result<u16, DeserializeError> {
    Ok(u16::from_be_bytes(read_n_bytes(buffer)?))
}

/// Deserializes an [`i8`] payload.
///
/// # Errors
/// Read `Errors` section of [`read_n_bytes()`].
fn deserrialize_i8(buffer: &mut Cursor<&[u8]>) -> Result<i8, DeserializeError> {
    Ok(i8::from_be_bytes(read_n_bytes(buffer)?))
}

/// Deserializes an [`u8`] payload.
///
/// # Errors
/// Read `Errors` section of [`read_n_bytes()`].
fn deserialize_byte(buffer: &mut Cursor<&[u8]>) -> Result<u8, DeserializeError> {
    Ok(u8::from_be_bytes(read_n_bytes(buffer)?))
}

/// Reads `N` bytes from the cursor into a statically allocated array.
///
/// # Errors
/// Returns an [`Exhausted`](DeserializeError::Exhausted) error if less than `N` bytes were read.
fn read_n_bytes<const N: usize>(buffer: &mut Cursor<&[u8]>) -> Result<[u8; N], DeserializeError> {
    let mut result = [0; N];
    buffer
        .read_exact(&mut result)
        .map_err(|_| DeserializeError::Exhausted(N))?;
    Ok(result)
}
