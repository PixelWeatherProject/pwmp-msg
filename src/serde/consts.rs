//! Internal constants for serialization and deserialization.

/// Magic start byte.
pub const MAGIC_START_BYTE: u8 = 0xFA;

/// Magic end byte.
pub const MAGIC_END_BYTE: u8 = 0xED;

/// Byte for representing that an optional value is empty.
pub const OPTIONAL_EMPTY: u8 = 0;

/// Byte for representing that an optional value has a value.
pub const OPTIONAL_EXIST: u8 = 1;
