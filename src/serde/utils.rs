//! Utility/helper functions for de/serialization.

use super::consts;
use super::error::Deserialize as DeserializeError;
use crate::version::Version;

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

/// Read next available byte from the iterator.
pub fn next_byte(bytes: &mut impl Iterator<Item = u8>) -> Result<u8, super::error::Deserialize> {
    bytes
        .next()
        .ok_or(super::error::Deserialize::ExpectedOneMoreByte)
}

/// Read next available byte from the iterator.
pub fn next_bytes<const N: usize>(
    bytes: &mut impl Iterator<Item = u8>,
) -> Result<[u8; N], super::error::Deserialize> {
    let mut array = [0; N];
    let mut index = 0;

    for _ in 0..N {
        match bytes.next() {
            Some(b) => {
                array[index] = b;
                index += 1;
            }
            None => {
                return Err(super::error::Deserialize::ExpectedMoreBytes {
                    expected: N,
                    got: index,
                });
            }
        }
    }

    Ok(array)
}

/// Returns whether the next optional in the buffer is empty or not.
pub fn deserialize_optional<I, T, F>(
    bytes: &mut I,
    value_deserializer: F,
) -> Result<Option<T>, DeserializeError>
where
    I: Iterator<Item = u8>,
    F: FnOnce(&mut I) -> Result<T, DeserializeError>,
{
    if !deserialize_bool(bytes)? {
        return Ok(None);
    }

    let value = value_deserializer(bytes)?;
    Ok(Some(value))
}

/// Deserialize an f32 value from bytes.
pub fn deserialize_f32(bytes: &mut impl Iterator<Item = u8>) -> Result<f32, DeserializeError> {
    let value_bytes = next_bytes(bytes)?;
    Ok(f32::from_ne_bytes(value_bytes))
}

/// Deserialize a usize value from bytes.
pub fn deserialize_usize(bytes: &mut impl Iterator<Item = u8>) -> Result<usize, DeserializeError> {
    let value_bytes = next_bytes(bytes)?;
    Ok(usize::from_ne_bytes(value_bytes))
}

/// Deserialize a u16 value from bytes.
pub fn deserialize_u16(bytes: &mut impl Iterator<Item = u8>) -> Result<u16, DeserializeError> {
    let value_bytes = next_bytes(bytes)?;
    Ok(u16::from_ne_bytes(value_bytes))
}

/// Deserialize an i8 value from bytes.
pub fn deserialize_i8(bytes: &mut impl Iterator<Item = u8>) -> Result<i8, DeserializeError> {
    let value_bytes = next_bytes(bytes)?;
    Ok(i8::from_ne_bytes(value_bytes))
}

/// Deserialize a blob (byte vector) from bytes.
pub fn deserialize_blob(
    bytes: &mut impl Iterator<Item = u8>,
) -> Result<Box<[u8]>, DeserializeError> {
    let len = deserialize_usize(bytes)?;

    Ok(core::iter::repeat_with(|| bytes.next().unwrap())
        .take(len)
        .collect())
}

/// Deserialize a string from bytes.
pub fn deserialize_string(
    bytes: &mut impl Iterator<Item = u8>,
) -> Result<Box<str>, DeserializeError> {
    let str_bytes = deserialize_blob(bytes)?.to_vec();
    let string = String::from_utf8(str_bytes)?.into_boxed_str();
    Ok(string)
}

/// Deserialize a version from bytes.
pub fn deserialize_version(
    bytes: &mut impl Iterator<Item = u8>,
) -> Result<Version, DeserializeError> {
    let values: [u8; 3] = next_bytes(bytes)?;
    Ok(Version::new(values[0], values[1], values[2]))
}

/// Deserialize a boolean value from bytes.
pub fn deserialize_bool(bytes: &mut impl Iterator<Item = u8>) -> Result<bool, DeserializeError> {
    match next_byte(bytes)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(DeserializeError::IllegalBoolean(other)),
    }
}
