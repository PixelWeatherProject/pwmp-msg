//! unfinished

mod deserializer;
mod serializer;

/// Alias to the integer type used to define the length of a bytes object.
pub type BytesLength = u16;

pub use deserializer::deserialize;
pub use serializer::serialize;
