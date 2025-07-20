//! A Serialization and deserialization implementation for PWMP.

use super::{request::Request, response::Response, Message, MessageContent};
use crate::serde::deserializer::Deserializable;
use error::Deserialize as DeserializeError;
use serializer::Serializable;

pub mod consts;
mod deserializer;
pub mod error;
mod serializer;
mod utils;

/// Serialize a message.
pub fn serialize(msg: Message) -> Box<[u8]> {
    let mut buffer = Vec::with_capacity(128);

    // push the message ID
    msg.id.serialize(&mut buffer);

    // push the message type (req/res)
    msg.type_id().serialize(&mut buffer);

    match msg.content {
        MessageContent::Request(req) => {
            req.serialize(&mut buffer);
        }
        MessageContent::Response(res) => {
            res.serialize(&mut buffer);
        }
    }

    // end
    buffer.into_boxed_slice()
}

/// Deserialize a message.
pub fn deserialize(bytes: &[u8]) -> Result<Message, DeserializeError> {
    let mut bytes = bytes.iter().copied();

    // get the message ID
    let msg_id = u32::deserialize(&mut bytes)?;

    // get the message type (req/res)
    let msg_type = utils::next_byte(&mut bytes)?;

    let message = match msg_type {
        consts::MSG_KIND_REQUEST => {
            let req = Request::deserialize(&mut bytes)?;
            Ok(Message::new_request(req, msg_id))
        }
        consts::MSG_KIND_RESPONSE => {
            let res = Response::deserialize(&mut bytes)?;
            Ok(Message::new_response(res, msg_id))
        }
        _ => Err(DeserializeError::IllegalMessageType(msg_type)),
    }?;

    // at this point we should have processed every byte in the array
    // there should be no more bytes returned by the iterator, otherwise we throw an error
    let leftover = bytes.count();
    if leftover != 0 {
        return Err(DeserializeError::ExtraDataLeft(leftover));
    }

    Ok(message)
}
