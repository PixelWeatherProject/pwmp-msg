//! unfinished

use super::BytesLength;
use crate::{request::Request, response::Response, version::Version, Message, MessageContent};
use thiserror::Error;

/// unfinished
#[derive(Debug, Error)]
pub enum SerializeError {
    /// Bytes object (byte array or string) is too large.
    #[error("Bytes object is too large ({0} bytes) to serialize")]
    BytesObjectTooLarge(usize),
}

/// unfinished
pub fn serialize(message: Message) -> Result<Box<[u8]>, SerializeError> {
    let mut buffer = Vec::with_capacity(2);

    buffer.extend_from_slice(&message.id.to_be_bytes());

    match message.content {
        MessageContent::Request(req) => serialize_request(req, &mut buffer)?,
        MessageContent::Response(res) => serialize_response(res, &mut buffer)?,
    }

    Ok(buffer.into_boxed_slice())
}

/// unfinished
pub fn serialize_request(req: Request, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
    buffer.push(Message::MSG_ID_REQUEST);

    match req {
        Request::Ping => buffer.push(Request::MSG_ID_PING),
        Request::Handshake { mac } => {
            buffer.push(Request::MSG_ID_HANDSHAKE);
            for i in 0..6 {
                buffer.push(mac[i]);
            }
        }
        Request::PostResults {
            temperature,
            humidity,
            air_pressure,
        } => {
            buffer.push(Request::MSG_ID_POST_RESULTS);
            buffer.extend_from_slice(&temperature.to_be_bytes());
            buffer.push(humidity);

            match air_pressure {
                None => buffer.push(0),
                Some(val) => {
                    buffer.push(1);
                    buffer.extend_from_slice(&val.to_be_bytes());
                }
            }
        }
        Request::PostStats {
            battery,
            wifi_ssid,
            wifi_rssi,
        } => {
            buffer.push(Request::MSG_ID_POST_STATS);
            buffer.extend_from_slice(&battery.to_be_bytes());
            serialize_string(&wifi_ssid, buffer)?;
            buffer.extend_from_slice(&wifi_rssi.to_be_bytes());
        }
        Request::SendNotification(content) => {
            buffer.push(Request::MSG_ID_SEND_NOTIFICATION);
            serialize_string(&content, buffer)?;
        }
        Request::GetSettings => {
            buffer.push(Request::MSG_ID_GET_SETTINGS);
        }
        Request::UpdateCheck(version) => {
            buffer.push(Request::MSG_ID_UPDATE_CHECK);
            serialize_version(version, buffer);
        }
        Request::NextUpdateChunk(chunk) => {
            buffer.push(Request::MSG_ID_NEXT_UPDATE_CHUNK);
            buffer.extend_from_slice(&chunk.to_be_bytes());
        }
        Request::ReportFirmwareUpdate(success) => {
            buffer.push(Request::MSG_ID_REPORT_FIRMWARE_UPDATE);
            buffer.push(u8::from(success));
        }
        Request::Bye => {
            buffer.push(Request::MSG_ID_BYE);
        }
    }

    Ok(())
}

/// unfinished
pub fn serialize_response(res: Response, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
    buffer.push(Message::MSG_ID_RESPONSE);

    match res {
        Response::Pong => buffer.push(Response::MSG_ID_PONG),
        Response::Ok => buffer.push(Response::MSG_ID_OK),
        Response::Reject => buffer.push(Response::MSG_ID_REJECT),
        Response::InvalidRequest => buffer.push(Response::MSG_ID_INVALID_REQUEST),
        Response::RateLimitExceeded => buffer.push(Response::MSG_ID_RATE_LIMIT_EXCEEDED),
        Response::InternalServerError => buffer.push(Response::MSG_ID_INTERNAL_SERVER_ERROR),
        Response::Stalling => buffer.push(Response::MSG_ID_STALLING),
        Response::FirmwareUpToDate => buffer.push(Response::MSG_ID_FIRMWARE_UP_TO_DATE),
        Response::UpdateAvailable(version) => {
            buffer.push(Response::MSG_ID_UPDATE_AVAILABLE);
            serialize_version(version, buffer);
        }
        Response::UpdatePart(items) => {
            buffer.push(Response::MSG_ID_UPDATE_PART);
            serilaize_bytes(&items, buffer)?;
        }
        Response::UpdateEnd => buffer.push(Response::MSG_ID_UPDATE_END),
        Response::Settings(node_settings) => {
            buffer.push(Response::MSG_ID_SETTINGS);

            match node_settings {
                None => buffer.push(0),
                Some(val) => {
                    buffer.push(1);
                    buffer.push(u8::from(val.battery_ignore));
                    buffer.push(u8::from(val.ota));
                    buffer.extend_from_slice(&val.sleep_time.to_be_bytes());
                    buffer.push(u8::from(val.sbop));
                    buffer.push(u8::from(val.mute_notifications));
                }
            }
        }
    }

    Ok(())
}

/// unfinished
fn serilaize_bytes(val: &[u8], buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
    if val.len() > usize::from(BytesLength::MAX) {
        return Err(SerializeError::BytesObjectTooLarge(val.len()));
    }

    // SAFETY: The length is checked above so it won't exceed what `BytesLength` can hold.
    let size = unsafe { BytesLength::try_from(val.len()).unwrap_unchecked() };

    buffer.extend_from_slice(&size.to_be_bytes());
    buffer.extend_from_slice(val);

    Ok(())
}

/// unfinished
fn serialize_string(val: &str, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
    serilaize_bytes(val.as_bytes(), buffer)
}

/// unfinished
fn serialize_version(val: Version, buffer: &mut Vec<u8>) {
    buffer.push(val.major());
    buffer.push(val.middle());
    buffer.push(val.minor());
}
