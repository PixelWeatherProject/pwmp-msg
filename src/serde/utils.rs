//! Utility/helper functions for de/serialization.

use crate::version::Version;

use super::consts;

/// Serialize a blob.
pub fn serialize_blob(val: &[u8], buffer: &mut Vec<u8>) {
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

/// Deserialize an f32 value from bytes.
pub fn deserialize_f32(bytes: &mut impl Iterator<Item = u8>) -> f32 {
    let value_bytes = core::array::from_fn(|_| bytes.next().unwrap());
    f32::from_ne_bytes(value_bytes)
}

/// Deserialize a usize value from bytes.
pub fn deserialize_usize(bytes: &mut impl Iterator<Item = u8>) -> usize {
    let value_bytes = core::array::from_fn(|_| bytes.next().unwrap());
    usize::from_ne_bytes(value_bytes)
}

/// Deserialize a u16 value from bytes.
pub fn deserialize_u16(bytes: &mut impl Iterator<Item = u8>) -> u16 {
    let value_bytes = core::array::from_fn(|_| bytes.next().unwrap());
    u16::from_ne_bytes(value_bytes)
}

/// Deserialize an i8 value from bytes.
pub fn deserialize_i8(bytes: &mut impl Iterator<Item = u8>) -> i8 {
    let value_bytes = core::array::from_fn(|_| bytes.next().unwrap());
    i8::from_ne_bytes(value_bytes)
}

/// Deserialize a blob (byte vector) from bytes.
pub fn deserialize_blob(bytes: &mut impl Iterator<Item = u8>) -> Vec<u8> {
    let len = deserialize_usize(bytes);
    core::iter::repeat_with(|| bytes.next().unwrap())
        .take(len)
        .collect()
}

/// Deserialize a string from bytes.
pub fn deserialize_string(bytes: &mut impl Iterator<Item = u8>) -> Box<str> {
    String::from_utf8(deserialize_blob(bytes))
        .unwrap()
        .into_boxed_str()
}

/// Deserialize a version from bytes.
pub fn deserialize_version(bytes: &mut impl Iterator<Item = u8>) -> Version {
    let values: [u8; 3] = core::array::from_fn(|_| bytes.next().unwrap());
    Version::new(values[0], values[1], values[2])
}

/// Deserialize a boolean value from bytes.
pub fn deserialize_bool(bytes: &mut impl Iterator<Item = u8>) -> bool {
    bytes.next().unwrap() == 1
}
