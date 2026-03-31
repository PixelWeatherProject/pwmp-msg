//! unfinished

use thiserror::Error;

mod deserializer;
mod serializer;

/// Alias to the integer type used to define the length of a bytes object.
pub type BytesLength = u16;

/// unfinished
#[derive(Debug, Error)]
pub enum SerdeError {
    /// Serialization error.
    #[error("Failed to serialize: {0}")]
    Serialize(#[from] SerializeError),

    /// Deserialization error.
    #[error("Failed to deserialize: {0}")]
    Deserialize(#[from] DeserializeError),
}

pub use deserializer::{deserialize, DeserializeError};
pub use serializer::{serialize, SerializeError};
