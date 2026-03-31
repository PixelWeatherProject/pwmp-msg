//! unfinished

use crate::Message;
use thiserror::Error;

/// unfinished
#[derive(Debug, Error)]
pub enum SerializeError {}

/// unfinished
pub fn serialize(message: Message) -> Result<Box<[u8]>, SerializeError> {
    todo!()
}
