//! Serialization and deserialization utilities.

mod deserializer;
mod serializer;

/// Alias to the integer type used to define the length of a bytes object.
pub type BytesLength = u16;

/// Byte representing an [`Option::Some(...)`](Option::Some).
pub const OPTIONAL_VALUE_EXISTS: u8 = 1;
/// Byte representing an [`Option::None`](Option::None).
pub const OPTIONAL_VALUE_VACANT: u8 = 0;

pub use deserializer::deserialize;
pub use serializer::serialize;
