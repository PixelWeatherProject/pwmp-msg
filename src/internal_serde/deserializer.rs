//! unfinished

use super::BytesLength;
use crate::{
    mac::Mac, request::Request, response::Response, settings::NodeSettings, version::Version,
    Message,
};
use bytes::Buf;
use std::io::{Cursor, Read};
use thiserror::Error;

/// unfinished
#[derive(Debug, Error)]
pub enum DeserializeError {
    /// Empty bytes buffer was provided
    #[error("Empty bytes buffer cannot be deserialized")]
    EmptyBuf,

    /// Not enough bytes could be read from the buffer
    #[error("Buffer was exhausted while reading: {0}")]
    Exhausted(#[from] bytes::TryGetError),

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

/// unfinished
pub fn deserialize(bytes: &[u8]) -> Result<Message, DeserializeError> {
    if bytes.is_empty() {
        return Err(DeserializeError::EmptyBuf);
    }

    let mut buffer = Cursor::new(bytes);
    let mid = buffer.try_get_u32()?;
    let kind = buffer.try_get_u8()?;

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

    Ok(message)
}

/// unfinished
fn deserialize_request(buffer: &mut Cursor<&[u8]>) -> Result<Request, DeserializeError> {
    let variant = buffer.try_get_u8()?;

    match variant {
        Request::MSG_ID_PING => Ok(Request::Ping),
        Request::MSG_ID_HANDSHAKE => {
            let mut mac = Mac::new(0, 0, 0, 0, 0, 0);

            for i in 0..6 {
                mac[i] = buffer.try_get_u8()?;
            }

            Ok(Request::Handshake { mac })
        }
        Request::MSG_ID_POST_RESULTS => {
            let temperature = buffer.try_get_f32()?;
            let humidity = buffer.try_get_u8()?;
            let air_pressure = match buffer.try_get_u8()? {
                0 => None,
                1 => Some(buffer.try_get_u16()?),
                other => return Err(DeserializeError::IllegalOptionalIdentifier(other)),
            };

            Ok(Request::PostResults {
                temperature,
                humidity,
                air_pressure,
            })
        }
        Request::MSG_ID_POST_STATS => {
            let battery = buffer.try_get_f32()?;
            let wifi_ssid = deserialize_string(buffer)?;
            let wifi_rssi = buffer.try_get_i8()?;

            Ok(Request::PostStats {
                battery,
                wifi_ssid,
                wifi_rssi,
            })
        }
        Request::MSG_ID_SEND_NOTIFICATION => {
            let content = deserialize_string(buffer)?;
            Ok(Request::SendNotification(content))
        }
        Request::MSG_ID_GET_SETTINGS => Ok(Request::GetSettings),
        Request::MSG_ID_UPDATE_CHECK => {
            let major = buffer.try_get_u8()?;
            let middle = buffer.try_get_u8()?;
            let minor = buffer.try_get_u8()?;

            Ok(Request::UpdateCheck(Version::new(major, middle, minor)))
        }
        Request::MSG_ID_NEXT_UPDATE_CHUNK => {
            let size = buffer.try_get_u32()?;
            Ok(Request::NextUpdateChunk(size))
        }
        Request::MSG_ID_REPORT_FIRMWARE_UPDATE => {
            let success = deserialize_bool(buffer)?;
            Ok(Request::ReportFirmwareUpdate(success))
        }
        Request::MSG_ID_BYE => Ok(Request::Bye),
        _ => Err(DeserializeError::IllegalRequestVariantIdentifier(variant)),
    }
}

/// unfinished
fn deserialize_response(buffer: &mut Cursor<&[u8]>) -> Result<Response, DeserializeError> {
    let variant = buffer.try_get_u8()?;

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
            let major = buffer.try_get_u8()?;
            let middle = buffer.try_get_u8()?;
            let minor = buffer.try_get_u8()?;

            Ok(Response::UpdateAvailable(Version::new(
                major, middle, minor,
            )))
        }
        Response::MSG_ID_UPDATE_PART => {
            let blob = deserialize_bytes(buffer)?;
            Ok(Response::UpdatePart(blob))
        }
        Response::MSG_ID_UPDATE_END => Ok(Response::UpdateEnd),
        Response::MSG_ID_SETTINGS => match buffer.try_get_u8()? {
            0 => Ok(Response::Settings(None)),
            1 => {
                let battery_ignore = deserialize_bool(buffer)?;
                let ota = deserialize_bool(buffer)?;
                let sleep_time = buffer.try_get_u16()?;
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

/// unfinished
fn deserialize_bytes(buffer: &mut Cursor<&[u8]>) -> Result<Box<[u8]>, DeserializeError> {
    let size: BytesLength = buffer.try_get_u16()?;
    let mut content = vec![0; usize::from(size)];
    buffer.read_exact(&mut content)?;

    Ok(content.into_boxed_slice())
}

/// unfinished
fn deserialize_string(buffer: &mut Cursor<&[u8]>) -> Result<Box<str>, DeserializeError> {
    let bytes = deserialize_bytes(buffer)?;
    Ok(String::from_utf8(bytes.to_vec())?.into_boxed_str())
}

/// unfinished
fn deserialize_bool(buffer: &mut Cursor<&[u8]>) -> Result<bool, DeserializeError> {
    match buffer.try_get_u8()? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(DeserializeError::IllegalBooleanValue(other)),
    }
}
