//! unfinished

use super::BytesLength;
use crate::{request::Request, response::Response, Message, MessageContent};
use bytes::BufMut;
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
    let mut buffer = Vec::with_capacity(4);

    buffer.put_u32(message.id);

    match message.content {
        MessageContent::Request(req) => serialize_request(req, &mut buffer)?,
        MessageContent::Response(res) => serialize_response(res, &mut buffer)?,
    }

    Ok(buffer.into_boxed_slice())
}

/// unfinished
pub fn serialize_request(req: Request, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
    match req {
        Request::Ping => buffer.put_u8(Request::MSG_ID_PING),
        Request::Handshake { mac } => {
            buffer.put_u8(Request::MSG_ID_HANDSHAKE);
            for i in 0..6 {
                buffer.put_u8(mac[i]);
            }
        }
        Request::PostResults {
            temperature,
            humidity,
            air_pressure,
        } => {
            buffer.put_u8(Request::MSG_ID_POST_RESULTS);
            buffer.put_f32(temperature);
            buffer.put_u8(humidity);

            match air_pressure {
                None => buffer.put_u8(0),
                Some(val) => {
                    buffer.put_u8(1);
                    buffer.put_u16(val);
                }
            }
        }
        Request::PostStats {
            battery,
            wifi_ssid,
            wifi_rssi,
        } => {
            buffer.put_u8(Request::MSG_ID_POST_STATS);
            buffer.put_f32(battery);
            serialize_string(&wifi_ssid, buffer)?;
            buffer.put_i8(wifi_rssi);
        }
        Request::SendNotification(content) => {
            buffer.put_u8(Request::MSG_ID_SEND_NOTIFICATION);
            serialize_string(&content, buffer)?;
        }
        Request::GetSettings => {
            buffer.put_u8(Request::MSG_ID_GET_SETTINGS);
        }
        Request::UpdateCheck(version) => {
            buffer.put_u8(Request::MSG_ID_UPDATE_CHECK);
            buffer.put_u8(version.major());
            buffer.put_u8(version.middle());
            buffer.put_u8(version.minor());
        }
        Request::NextUpdateChunk(chunk) => {
            buffer.put_u8(Request::MSG_ID_NEXT_UPDATE_CHUNK);
            buffer.put_u32(chunk);
        }
        Request::ReportFirmwareUpdate(success) => {
            buffer.put_u8(Request::MSG_ID_REPORT_FIRMWARE_UPDATE);
            buffer.put_u8(u8::from(success));
        }
        Request::Bye => {
            buffer.put_u8(Request::MSG_ID_BYE);
        }
    }

    Ok(())
}

/// unfinished
pub fn serialize_response(res: Response, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
    match res {
        Response::Pong => buffer.put_u8(Response::MSG_ID_PONG),
        Response::Ok => buffer.put_u8(Response::MSG_ID_OK),
        Response::Reject => buffer.put_u8(Response::MSG_ID_REJECT),
        Response::InvalidRequest => buffer.put_u8(Response::MSG_ID_INVALID_REQUEST),
        Response::RateLimitExceeded => buffer.put_u8(Response::MSG_ID_RATE_LIMIT_EXCEEDED),
        Response::InternalServerError => buffer.put_u8(Response::MSG_ID_INTERNAL_SERVER_ERROR),
        Response::Stalling => buffer.put_u8(Response::MSG_ID_STALLING),
        Response::FirmwareUpToDate => buffer.put_u8(Response::MSG_ID_FIRMWARE_UP_TO_DATE),
        Response::UpdateAvailable(version) => {
            buffer.put_u8(Response::MSG_ID_UPDATE_AVAILABLE);
            buffer.put_u8(version.major());
            buffer.put_u8(version.middle());
            buffer.put_u8(version.minor());
        }
        Response::UpdatePart(items) => {
            buffer.put_u8(Response::MSG_ID_UPDATE_PART);
            serilaize_bytes(&items, buffer)?;
        }
        Response::UpdateEnd => buffer.put_u8(Response::MSG_ID_UPDATE_END),
        Response::Settings(node_settings) => {
            buffer.put_u8(Response::MSG_ID_SETTINGS);

            match node_settings {
                None => buffer.put_u8(0),
                Some(val) => {
                    buffer.put_u8(1);
                    buffer.put_u8(u8::from(val.battery_ignore));
                    buffer.put_u8(u8::from(val.ota));
                    buffer.put_u16(val.sleep_time);
                    buffer.put_u8(u8::from(val.sbop));
                    buffer.put_u8(u8::from(val.mute_notifications));
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

    buffer.put_u16(size);
    buffer.put(val);

    Ok(())
}

/// unfinished
fn serialize_string(val: &str, buffer: &mut Vec<u8>) -> Result<(), SerializeError> {
    serilaize_bytes(val.as_bytes(), buffer)
}
