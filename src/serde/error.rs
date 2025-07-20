//! Error types for de/serialization.

use thiserror::Error;

/// Deserialization error.
#[derive(Debug, Error)]
pub enum Deserialize {
    /// Expected to read one more byte from a buffer, but it's empty/exhausted.
    #[error("Expected one more byte in buffer, but there are no more available.")]
    ExpectedOneMoreByte,

    /// Expected `N` more bytes from a buffer, but got less/more.
    #[error("Expected {expected} more bytes from buffer, but got {got} instead.")]
    ExpectedMoreBytes {
        /// The amount of bytes we expected from the buffer.
        expected: usize,

        /// The amount of bytes we actually got from the buffer.
        got: usize,
    },

    /// Invalid boolean value.
    #[error("Expected to deserialize boolean value 0 or 1, but got {0} instead.")]
    IllegalBoolean(u8),

    /// Invalid message type identifier.
    #[error("Invalid message type '{0}'.")]
    IllegalMessageType(u8),

    /// Invalid request type identifier.
    #[error("Invalid request type '{0}'.")]
    IllegalRequestType(u8),

    /// Invalid response type identifier.
    #[error("Invalid response type '{0}'.")]
    IllegalResponseType(u8),

    /// Invalid UTF-8 string.
    #[error("Failed to validate UTF-8 string sequence: '{0}'.")]
    IllegalUtf8String(#[from] std::string::FromUtf8Error),

    /// The buffer should be empty, but it still contains extra unprocessed and unnecessary data.
    #[error("Expected buffer to be empty, but it still has {0} more bytes")]
    ExtraDataLeft(usize),
}
