pub use bytes::Bytes;
pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

pub mod aliases;
mod kick;
pub mod mac;
pub mod request;
pub mod response;
pub mod settings;
pub mod version;

/// A Message object.
/// Can either be a request or a response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Message {
    /// Server requested data from a client or vice-versa.
    Request(request::Request),
    /// Server responded to a request from a client or vice-versa.
    Response(response::Response),
}

impl Message {
    /// Serialize the message into raw bytes.
    ///
    /// # Panics
    /// This will panic if the message could not be serialized.
    #[must_use]
    pub fn serialize(self) -> Bytes {
        postcard::to_stdvec(&self).unwrap().into()
    }

    /// Deserialize a message from raw bytes.
    #[must_use]
    pub fn deserialize(bytes: &Bytes) -> Option<Self> {
        postcard::from_bytes(bytes).ok()
    }

    /// Returns a reference to the contained [`Request`].
    /// If the message contains a [`Response`] instead, `None` is returned.
    #[must_use]
    pub const fn request(&self) -> Option<&request::Request> {
        if let Self::Request(req) = self {
            Some(req)
        } else {
            None
        }
    }

    /// Returns a reference to the contained [`Response`].
    /// If the message contains a [`Request`] instead, `None` is returned.
    #[must_use]
    pub const fn response(&self) -> Option<&response::Response> {
        if let Self::Response(resp) = self {
            Some(resp)
        } else {
            None
        }
    }

    /// Similar to [`request()`](Self::request), but consumes the message itself.
    #[must_use]
    pub fn take_request(self) -> Option<request::Request> {
        if let Self::Request(req) = self {
            Some(req)
        } else {
            None
        }
    }

    /// Similar to [`response()`](Self::response), but consumes the message itself.
    #[must_use]
    pub fn take_response(self) -> Option<response::Response> {
        if let Self::Response(resp) = self {
            Some(resp)
        } else {
            None
        }
    }

    /// Returns the length of the message if it was serialized.
    ///
    /// # Panics
    /// This will panic if the message could not be serialized.
    ///
    /// # Example
    /// ```rust
    /// # use pwmp_msg::{Message, response::Response, request::Request};
    /// let ping = Message::Request(Request::Ping);
    /// let pong = Message::Response(Response::Pong);
    ///
    /// assert_eq!(ping.size(), 2);
    /// assert_eq!(pong.size(), 2);
    /// ```
    #[must_use]
    pub fn size(&self) -> usize {
        postcard::to_stdvec(self).unwrap().len()
    }
}
