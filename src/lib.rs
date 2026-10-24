#![deny(missing_docs, clippy::missing_docs_in_private_items)]

//! Pixel-Weather Messaging Protocol core library.
//!
//! This library contains the definitions of all possible messages in the PWMP.

pub use postcard;
use serde::{Deserialize, Serialize};

pub mod aliases;
pub mod mac;
pub mod request;
pub mod response;
pub mod settings;
pub mod version;

/// Message ID type.
pub type MsgId = u8;

/// A Message object.
/// Can either be a request or a response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    /// Unique ID of this message.
    ///
    /// This ID is used to check if the message is **not** a duplicate.
    /// Due to the limited amount of values, the client and the server implementations do not need
    /// to keep track of every ID of every message. Technically only the ID of the previous message
    /// needs to be remembered.
    ///
    /// The server and client should keep a short-term cache of the sent/received IDs
    /// to determinte if the same message hasn't been duplicated.
    id: MsgId,

    /// Actual content of the message, which can be either a request or a response.
    content: MessageContent,
}

/// A wrapper for the contents of the message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum MessageContent {
    /// Client (node) has requested the server to perform an operation.
    Request(request::Request),

    /// Server has responded to a client's request.
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
    /// let request = Request::GetSettings;
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
    /// let response = Response::Ok;
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
    /// let request = Request::GetSettings;
    /// let message = Message::new_request(request, id);
    /// let bytes = message.serialize();
    /// ```
    ///
    /// ```rust
    /// use pwmp_msg::{Message, response::Response};
    ///
    /// let id = 1;
    /// let response = Response::Ok;
    /// let message = Message::new_response(response, id);
    /// let bytes = message.serialize();
    /// ```
    ///
    /// # Errors
    /// Returns a [`postcard::Result`], which will contain an error if the serialization fails.
    pub fn serialize(self) -> postcard::Result<Box<[u8]>> {
        postcard::to_allocvec(&self).map(Vec::into_boxed_slice)
    }

    /// Serialize the message into raw bytes, writing them to the specified buffer without allocating.
    ///
    /// This is similar to [`serialize()`](Self::serialize), but requires the caller
    /// to provide a valid, large enough buffer.
    ///
    /// This may be useful for applications that repeatedly serialize messages.
    ///
    /// # Examples
    /// ```rust
    /// use pwmp_msg::{Message, request::Request};
    ///
    /// let id = 1;
    /// let request = Request::GetSettings;
    /// let message = Message::new_request(request, id);
    ///
    /// let mut buffer = [0u8; 4];
    /// let bytes = message.serialize_into(&mut buffer).unwrap();
    ///
    /// assert_eq!(bytes, &[1, 0, 4]);
    /// ```
    ///
    /// In the following example, the buffer is too small to contain the data, so it fails:
    /// ```rust
    /// use pwmp_msg::{Message, request::Request};
    ///
    /// let id = 1;
    /// let request = Request::SendNotification("Hello, world!".into());
    /// let message = Message::new_request(request, id);
    ///
    /// let mut buffer = [0u8; 4];
    /// let error = message.serialize_into(&mut buffer).unwrap_err();
    ///
    /// assert_eq!(error.to_string(), "The serialize buffer is full");
    /// ```
    ///
    /// # Errors
    /// Returns a [`postcard::Result`], which will contain an error if the serialization fails.
    pub fn serialize_into(self, buffer: &mut [u8]) -> postcard::Result<&mut [u8]> {
        postcard::to_slice(&self, buffer)
    }

    /// Deserialize a message from raw bytes.
    ///
    /// # Example
    /// ```rust
    /// use pwmp_msg::{Message, request::Request};
    ///
    /// let id = 1;
    /// let request = Request::GetSettings;
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
    /// let response = Response::Ok;
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
    /// let original_request = Request::GetSettings;
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
    /// let response = Response::Ok;
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
    /// let original_response = Response::Ok;
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
    /// let request = Request::GetSettings;
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
    /// let original_request = Request::GetSettings;
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
    /// let response = Response::Ok;
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
    /// let original_response = Response::Ok;
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
    /// let request = Request::GetSettings;
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
    /// let request = Request::GetSettings;
    /// let message = Message::new_request(request, id);
    ///
    /// assert_eq!(message.id(), id);
    /// ```
    ///
    /// ```rust
    /// use pwmp_msg::{Message, response::Response};
    ///
    /// let id = 1;
    /// let response = Response::Ok;
    /// let message = Message::new_response(response, id);
    ///
    /// assert_eq!(message.id(), id);
    /// ```
    #[must_use]
    pub const fn id(&self) -> MsgId {
        self.id
    }
}
