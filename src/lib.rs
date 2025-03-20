pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

pub mod aliases;
pub mod mac;
pub mod request;
pub mod response;
pub mod settings;
pub mod version;

/// Message ID type.
pub type MsgId = u32;

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
    id: MsgId,

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
    /// Wrap a request and assign the given ID to the message.
    ///
    /// # Example
    /// ```rust
    /// use pwmp_msg::{Message, request::Request};
    ///
    /// let id = 1;
    /// let request = Request::Ping;
    /// let message = Message::new_request(request.clone(), id);
    ///
    /// assert_eq!(message.id(), id);
    /// assert_eq!(message.take_request(), Some(request));
    /// ```
    #[must_use]
    pub const fn new_request(req: request::Request, id: MsgId) -> Self {
        Self {
            id,
            content: MessageContent::Request(req),
        }
    }

    /// Wrap a response and assign the given ID to the message.
    ///
    /// # Example
    /// ```rust
    /// use pwmp_msg::{Message, response::Response};
    ///
    /// let id = 1;
    /// let response = Response::Pong;
    /// let message = Message::new_response(response.clone(), id);
    ///
    /// assert_eq!(message.id(), id);
    /// assert_eq!(message.take_response(), Some(response));
    /// ```
    #[must_use]
    pub const fn new_response(res: response::Response, id: MsgId) -> Self {
        Self {
            id,
            content: MessageContent::Response(res),
        }
    }

    /// Serialize the message into raw bytes.
    ///
    /// # Example
    /// ```rust
    /// use pwmp_msg::{Message, request::Request};
    ///
    /// let id = 1;
    /// let request = Request::Ping;
    /// let message = Message::new_request(request, id);
    /// let bytes = message.serialize();
    /// ```
    ///
    /// ```rust
    /// use pwmp_msg::{Message, response::Response};
    ///
    /// let id = 1;
    /// let response = Response::Pong;
    /// let message = Message::new_response(response, id);
    /// let bytes = message.serialize();
    /// ```
    ///
    /// # Panics
    /// This will panic if the message could not be serialized.
    #[must_use]
    pub fn serialize(self) -> Box<[u8]> {
        postcard::to_stdvec(&self).unwrap().into_boxed_slice()
    }

    /// Deserialize a message from raw bytes.
    ///
    /// # Example
    /// ```rust
    /// use pwmp_msg::{Message, request::Request};
    ///
    /// let id = 1;
    /// let request = Request::Ping;
    /// let message = Message::new_request(request, id);
    ///
    /// let bytes = message.clone().serialize();
    /// let original_message = Message::deserialize(&bytes).unwrap();
    ///
    /// assert_eq!(message, original_message);
    /// ```
    ///
    /// ```rust
    /// use pwmp_msg::{Message, response::Response};
    ///
    /// let id = 1;
    /// let response = Response::Pong;
    /// let message = Message::new_response(response, id);
    ///
    /// let bytes = message.clone().serialize();
    /// let original_message = Message::deserialize(&bytes).unwrap();
    ///
    /// assert_eq!(message, original_message);
    /// ```
    #[must_use]
    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        postcard::from_bytes(bytes).ok()
    }

    /// Returns a reference to the contained [`Request`].
    /// If the message contains a [`Response`] instead, `None` is returned.
    ///
    /// ```rust
    /// use pwmp_msg::{Message, request::Request};
    ///
    /// let id = 1;
    /// let original_request = Request::Ping;
    /// let message = Message::new_request(original_request.clone(), id);
    /// let wrapped_request = message.request();
    ///
    /// assert_eq!(wrapped_request, Some(&original_request));
    /// ```
    ///
    /// ```rust
    /// use pwmp_msg::{Message, response::Response};
    ///
    /// let id = 1;
    /// let response = Response::Pong;
    /// let message = Message::new_response(response, id);
    ///
    /// assert_eq!(message.request(), None);
    /// ```
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
    ///
    /// ```rust
    /// use pwmp_msg::{Message, response::Response};
    ///
    /// let id = 1;
    /// let original_response = Response::Pong;
    /// let message = Message::new_response(original_response.clone(), id);
    /// let wrapped_response = message.response();
    ///
    /// assert_eq!(wrapped_response, Some(&original_response));
    /// ```
    ///
    /// ```rust
    /// use pwmp_msg::{Message, request::Request};
    ///
    /// let id = 1;
    /// let request = Request::Ping;
    /// let message = Message::new_request(request, id);
    ///
    /// assert_eq!(message.response(), None);
    /// ```
    #[must_use]
    pub const fn response(&self) -> Option<&response::Response> {
        if let MessageContent::Response(res) = &self.content {
            Some(res)
        } else {
            None
        }
    }

    /// Similar to [`request()`](Self::request), but consumes the message itself.
    ///
    /// ```rust
    /// use pwmp_msg::{Message, request::Request};
    ///
    /// let id = 1;
    /// let original_request = Request::Ping;
    /// let message = Message::new_request(original_request.clone(), id);
    /// let wrapped_request = message.take_request();
    ///
    /// assert_eq!(wrapped_request, Some(original_request));
    /// ```
    ///
    /// ```rust
    /// use pwmp_msg::{Message, response::Response};
    ///
    /// let id = 1;
    /// let response = Response::Pong;
    /// let message = Message::new_response(response, id);
    ///
    /// assert_eq!(message.take_request(), None);
    /// ```
    #[must_use]
    pub fn take_request(self) -> Option<request::Request> {
        if let MessageContent::Request(req) = self.content {
            Some(req)
        } else {
            None
        }
    }

    /// Similar to [`response()`](Self::response), but consumes the message itself.
    ///
    /// ```rust
    /// use pwmp_msg::{Message, response::Response};
    ///
    /// let id = 1;
    /// let original_response = Response::Pong;
    /// let message = Message::new_response(original_response.clone(), id);
    /// let wrapped_response = message.take_response();
    ///
    /// assert_eq!(wrapped_response, Some(original_response));
    /// ```
    ///
    /// ```rust
    /// use pwmp_msg::{Message, request::Request};
    ///
    /// let id = 1;
    /// let request = Request::Ping;
    /// let message = Message::new_request(request, id);
    ///
    /// assert_eq!(message.take_response(), None);
    /// ```
    #[must_use]
    pub fn take_response(self) -> Option<response::Response> {
        if let MessageContent::Response(res) = self.content {
            Some(res)
        } else {
            None
        }
    }

    /// Returns the message ID.
    ///
    /// ```rust
    /// use pwmp_msg::{Message, request::Request};
    ///
    /// let id = 1;
    /// let request = Request::Ping;
    /// let message = Message::new_request(request, id);
    ///
    /// assert_eq!(message.id(), id);
    /// ```
    ///
    /// ```rust
    /// use pwmp_msg::{Message, response::Response};
    ///
    /// let id = 1;
    /// let response = Response::Pong;
    /// let message = Message::new_response(response, id);
    ///
    /// assert_eq!(message.id(), id);
    /// ```
    #[must_use]
    pub const fn id(&self) -> MsgId {
        self.id
    }
}
