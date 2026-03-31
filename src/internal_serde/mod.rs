//! unfinished

use thiserror::Error;

mod deserializer;
mod serializer;

/// unfinished
#[derive(Debug, Error)]
pub enum SerdeError {}

pub use deserializer::{deserialize, DeserializeError};
pub use serializer::{serialize, SerializeError};
