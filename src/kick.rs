use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum KickReason {
    /// An error occurred on the server side while processing the request.
    InternalServerError,

    /// Server reached maximum number of clients.
    CapacityReached,

    /// Invalid request.
    BadRequest,

    /// Client has sent too many requests.
    TooManyRequests,

    /// Unknown reason.
    Unspecified,
}
