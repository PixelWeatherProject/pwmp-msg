pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

pub mod aliases;
pub mod mac;
pub mod request;
pub mod response;
pub mod settings;
pub mod version;

/// A Message object.
/// Can either be a request or a response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    /// Unique ID of this message.
    ///
    /// Reliability of the unsigned 32-bit integer ID:
    /// - If two messages happen to have the same ID, as long as they're sent to two different clients, it's fine.
    /// - The client and the server won't usually exchange many messages.
    /// - If the client requests too many OTA update chunks, this might be problematic.
    ///
    /// The server and client should keep a short-term cache of the sent/received IDs
    /// to determinte if the same message hasn't been duplicated.
    id: u32,

    /// Actual content of the message, which can be either a request or a response.
    content: MessageContent,
}

/// A wrapper for the contents of the message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum MessageContent {
    Request(request::Request),
    Response(response::Response),
}

impl Message {
    pub const fn new_request(req: request::Request, id: u32) -> Self {
        Self {
            id,
            content: MessageContent::Request(req),
        }
    }

    pub const fn new_response(res: response::Response, id: u32) -> Self {
        Self {
            id,
            content: MessageContent::Response(res),
        }
    }

    /// Serialize the message into raw bytes.
    ///
    /// # Panics
    /// This will panic if the message could not be serialized.
    #[must_use]
    pub fn serialize(self) -> Box<[u8]> {
        postcard::to_stdvec(&self).unwrap().into_boxed_slice()
    }

    /// Deserialize a message from raw bytes.
    #[must_use]
    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        postcard::from_bytes(bytes).ok()
    }

    /// Returns a reference to the contained [`Request`].
    /// If the message contains a [`Response`] instead, `None` is returned.
    #[must_use]
    pub const fn request(&self) -> Option<&request::Request> {
        if let MessageContent::Request(req) = &self.content {
            Some(req)
        } else {
            None
        }
    }

    /// Returns a reference to the contained [`Response`].
    /// If the message contains a [`Request`] instead, `None` is returned.
    #[must_use]
    pub const fn response(&self) -> Option<&response::Response> {
        if let MessageContent::Response(res) = &self.content {
            Some(res)
        } else {
            None
        }
    }

    /// Similar to [`request()`](Self::request), but consumes the message itself.
    #[must_use]
    pub fn take_request(self) -> Option<request::Request> {
        if let MessageContent::Request(req) = self.content {
            Some(req)
        } else {
            None
        }
    }

    /// Similar to [`response()`](Self::response), but consumes the message itself.
    #[must_use]
    pub fn as_response(self) -> Option<response::Response> {
        if let MessageContent::Response(res) = self.content {
            Some(res)
        } else {
            None
        }
    }
}
