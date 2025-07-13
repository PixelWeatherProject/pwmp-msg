//! Trants and their implementations for de/serialization.

/// Trait for an object that is serializable.
pub trait Serializable {
    /// Serialize this object into a buffer of bytes.
    fn serialize_into(&self, buffer: &mut Vec<u8>);

    fn deserialize_from(&self, buffer: &mut Vec<u8>);
}
