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
pub enum Message {
    /// Server requested data from a client or vice-versa.
    Request(request::Request),
    /// Server responded to a request from a client or vice-versa.
    Response(response::Response),
}

impl Message {
    /// Serialize the message into raw bytes.
    #[must_use]
    pub fn serialize(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    /// Deserialize a message from raw bytes.
    #[must_use]
    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        bincode::deserialize(bytes).ok()
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
    pub fn as_request(self) -> Option<request::Request> {
        if let Self::Request(req) = self {
            Some(req)
        } else {
            None
        }
    }

    /// Similar to [`response()`](Self::response), but consumes the message itself.
    #[must_use]
    pub fn as_response(self) -> Option<response::Response> {
        if let Self::Response(resp) = self {
            Some(resp)
        } else {
            None
        }
    }

    /// Returns the length of the message if it was serialized.
    /// ```rust
    /// # use pwmp_msg::{Message, response::Response, request::Request};
    /// let ping = Message::Request(Request::Ping);
    /// let pong = Message::Response(Response::Pong);
    ///
    /// assert_eq!(ping.size(), 8);
    /// assert_eq!(pong.size(), 8);
    /// ```
    #[must_use]
    pub fn size(&self) -> usize {
        bincode::serialized_size(self).unwrap() as usize
    }
}
