//! unfinished

mod deserializer;
mod serializer;

/// unfinished
pub enum SerdeError {}

pub use deserializer::{deserialize, DeserializeError};
pub use serializer::{serialize, SerializeError};
