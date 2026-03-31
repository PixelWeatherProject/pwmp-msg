//! unfinished

use crate::Message;
use thiserror::Error;

/// unfinished
#[derive(Debug, Error)]
pub enum DeserializeError {}

/// unfinished
pub fn deserialize(bytes: &[u8]) -> Result<Message, DeserializeError> {
    todo!()
}
