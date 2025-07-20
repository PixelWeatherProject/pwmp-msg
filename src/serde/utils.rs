//! Utility/helper functions for de/serialization.

use super::DeserializeError;

/// Read next available byte from the iterator.
pub fn next_byte(bytes: &mut impl Iterator<Item = u8>) -> Result<u8, DeserializeError> {
    bytes.next().ok_or(DeserializeError::ExpectedOneMoreByte)
}

/// Read next available byte from the iterator.
pub fn next_bytes<const N: usize>(
    bytes: &mut impl Iterator<Item = u8>,
) -> Result<[u8; N], DeserializeError> {
    let mut array = [0; N];
    let mut index = 0;

    for _ in 0..N {
        match bytes.next() {
            Some(b) => {
                array[index] = b;
                index += 1;
            }
            None => {
                return Err(DeserializeError::ExpectedMoreBytes {
                    expected: N,
                    got: index,
                });
            }
        }
    }

    Ok(array)
}
