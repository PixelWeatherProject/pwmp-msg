//! unfinished

use super::BytesLength;
use crate::{mac::Mac, request::Request, Message};
use bytes::Buf;
use thiserror::Error;

/// unfinished
#[derive(Debug, Error)]
pub enum DeserializeError {
    /// Empty bytes buffer was provided
    #[error("Empty bytes buffer cannot be deserialized")]
    EmptyBuf,

    /// Not enought bytes could be read from the buffer
    #[error("Buffer was exhausted while reading: {0}")]
    Exhausted(#[from] bytes::TryGetError),

    /// Invalid message type identifier
    #[error("Invalid message type: {0}")]
    IllegalVariantIdentifier(u8),

    /// Invalid [`Request`] type identifier
    #[error("Invalid message type: {0}")]
    IllegalRequestVariantIdentifier(u8),

    /// Invalid optional value identifier
    #[error("Optional value identifier '{0}' is invalid")]
    IllegalOptionalIdentifier(u8),
}

/// unfinished
pub fn deserialize(mut bytes: &[u8]) -> Result<Message, DeserializeError> {
    if bytes.is_empty() {
        return Err(DeserializeError::EmptyBuf);
    }

    let mid = bytes.try_get_u32()?;
    let kind = bytes.try_get_u8()?;

    let message = match kind {
        Message::MSG_ID_REQUEST => {
            let req = deserialize_request(&mut bytes)?;
            Message::new_request(req, mid)
        }
        Message::MSG_ID_RESPONSE => {
            //let res = deserialize_response(&mut bytes)?;
            //Message::new_response(res, mid)
            todo!()
        }
        _ => return Err(DeserializeError::IllegalVariantIdentifier(kind)),
    };

    Ok(message)
}

/// unfinished
fn deserialize_request(buffer: &mut &[u8]) -> Result<Request, DeserializeError> {
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
        Request::MSG_ID_POST_STATS => todo!(),
        Request::MSG_ID_SEND_NOTIFICATION => todo!(),
        Request::MSG_ID_GET_SETTINGS => todo!(),
        Request::MSG_ID_UPDATE_CHECK => todo!(),
        Request::MSG_ID_NEXT_UPDATE_CHUNK => todo!(),
        Request::MSG_ID_REPORT_FIRMWARE_UPDATE => todo!(),
        Request::MSG_ID_BYE => todo!(),
        _ => Err(DeserializeError::IllegalRequestVariantIdentifier(variant)),
    }
}

/// unfinished
fn deserialize_bytes<'a>(buffer: &mut &'a [u8]) -> Result<&'a [u8], DeserializeError> {
    let size: BytesLength = buffer.try_get_u16()?;

    todo!()
}
