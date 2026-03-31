//! unfinished

use crate::Message;
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
    #[error("Invalid message type")]
    IllegalVariantIdentifier,
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
            todo!()
        }
        Message::MSG_ID_RESPONSE => {
            todo!()
        }
        _ => return Err(DeserializeError::IllegalVariantIdentifier),
    };

    Ok(message)
}
