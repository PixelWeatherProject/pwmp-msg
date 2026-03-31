//! unfinished

use thiserror::Error;

mod deserializer;
mod serializer;

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
