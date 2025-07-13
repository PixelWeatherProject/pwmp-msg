//! Utility/helper functions for de/serialization.

use super::consts;

/// Serialize a blob.
pub fn serialize_blob(val: &[u8], buffer: &mut Vec<u8>) {
    buffer.reserve(size_of::<usize>() + val.len());
    buffer.extend_from_slice(&val.len().to_ne_bytes());
    buffer.extend_from_slice(val);
}

/// Serialize an optinal.
pub fn serialize_optional<T, F>(val: Option<T>, value_serializer: F, buffer: &mut Vec<u8>)
where
    F: FnOnce(T, &mut Vec<u8>),
{
    match val {
        Some(inner) => {
            buffer.push(consts::OPTIONAL_EXIST);
            value_serializer(inner, buffer);
        }
        None => {
            buffer.push(consts::OPTIONAL_EMPTY);
        }
    }
}
