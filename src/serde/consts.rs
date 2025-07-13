//! Internal constants for serialization and deserialization.

/// Byte for representing that an optional value is empty.
pub const OPTIONAL_EMPTY: u8 = 0;

/// Byte for representing that an optional value has a value.
pub const OPTIONAL_EXIST: u8 = 1;
