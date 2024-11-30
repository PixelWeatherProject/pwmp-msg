use serde::{Deserialize, Serialize};

pub mod aliases;
pub mod mac;
pub mod request;
pub mod response;
pub mod settings;

/// A Message object.
/// Can either be a request or a response.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Message {
    /// Server requested data from a client or vice-versa.
    Request(request::Request),
    /// Server responded to a request from a client or vice-versa.
    Response(response::Response),
}
