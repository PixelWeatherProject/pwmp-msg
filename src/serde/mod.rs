//! A Serialization and deserialization implementation for PWMP.

use super::{request::Request, response::Response, Message, MessageContent};
use crate::{mac::Mac, serde::utils::next_byte, settings::NodeSettings};
use error::Deserialize as DeserializeError;

pub mod consts;
pub mod error;
mod utils;

/// Serialize a message.
pub fn serialize(msg: Message) -> Box<[u8]> {
    let mut buffer = Vec::with_capacity(128);

    // push the message ID
    buffer.extend_from_slice(&msg.id.to_ne_bytes());

    // push the message type (req/res)
    buffer.push(msg.type_id());

    match msg.content {
        MessageContent::Request(req) => {
            serialize_request(req, &mut buffer);
        }
        MessageContent::Response(res) => {
            serialize_response(res, &mut buffer);
        }
    }

    // end
    buffer.into_boxed_slice()
}

/// Serialize a request.
fn serialize_request(req: Request, buffer: &mut Vec<u8>) {
    buffer.push(req.type_id());

    match req {
        Request::Ping => {
            buffer.push(0); // first variant with empty values
        }
        Request::Handshake { mac } => {
            buffer.extend_from_slice(&[mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]]);
        }
        Request::PostResults {
            temperature,
            humidity,
            air_pressure,
        } => {
            buffer.extend_from_slice(&temperature.to_ne_bytes());
            buffer.push(humidity);

            utils::serialize_optional(
                air_pressure,
                |val, buffer| {
                    buffer.extend_from_slice(&val.to_ne_bytes());
                },
                buffer,
            );
        }
        Request::PostStats {
            battery,
            wifi_ssid,
            wifi_rssi,
        } => {
            buffer.extend_from_slice(&battery.to_ne_bytes());
            utils::serialize_blob(wifi_ssid.as_bytes(), buffer);
            buffer.extend_from_slice(&wifi_rssi.to_ne_bytes());
        }
        Request::SendNotification(content) => {
            utils::serialize_blob(content.as_bytes(), buffer);
        }
        Request::GetSettings => {
            buffer.push(1); // second variant with empty values
        }
        Request::UpdateCheck(current_ver) => {
            buffer.reserve(3); // prevent 3 separate allocations
            buffer.push(current_ver.major());
            buffer.push(current_ver.middle());
            buffer.push(current_ver.minor());
        }
        Request::NextUpdateChunk(size) => {
            buffer.extend_from_slice(&size.to_ne_bytes());
        }
        Request::ReportFirmwareUpdate(good) => {
            buffer.push(u8::from(good));
        }
        Request::Bye => {
            buffer.push(2); // third variant with empty values
        }
    }
}

/// Serialize a response.
fn serialize_response(res: Response, buffer: &mut Vec<u8>) {
    buffer.push(res.type_id());

    match res {
        Response::Pong => {
            buffer.push(0);
        }
        Response::Ok => {
            buffer.push(1);
        }
        Response::Reject => {
            buffer.push(2);
        }
        Response::InvalidRequest => {
            buffer.push(3);
        }
        Response::RateLimitExceeded => {
            buffer.push(4);
        }
        Response::InternalServerError => {
            buffer.push(5);
        }
        Response::Stalling => {
            buffer.push(6);
        }
        Response::FirmwareUpToDate => {
            buffer.push(7);
        }
        Response::UpdateAvailable(new_version) => {
            buffer.reserve(3); // prevent 3 separate allocations
            buffer.push(new_version.major());
            buffer.push(new_version.middle());
            buffer.push(new_version.minor());
        }
        Response::UpdatePart(blob) => {
            buffer.extend_from_slice(&blob);
        }
        Response::UpdateEnd => {
            buffer.push(8);
        }
        Response::Settings(settings) => {
            utils::serialize_optional(
                settings,
                |val, buffer| {
                    buffer.push(u8::from(val.battery_ignore));
                    buffer.push(u8::from(val.ota));
                    buffer.extend_from_slice(&val.sleep_time.to_ne_bytes());
                    buffer.push(u8::from(val.sbop));
                    buffer.push(u8::from(val.sbop));
                    buffer.push(u8::from(val.mute_notifications));
                },
                buffer,
            );
        }
    }
}

/// Deserialize a request.
pub fn deserialize_request(
    bytes: &mut impl Iterator<Item = u8>,
) -> Result<Request, DeserializeError> {
    // get the request type
    let req_type = utils::next_byte(bytes)?;

    match req_type {
        consts::REQ_KIND_PING => Ok(Request::Ping),
        consts::REQ_KIND_HANDSHAKE => {
            let octets = utils::next_bytes::<6>(bytes)?;
            let mac = Mac::new(
                octets[0], octets[1], octets[2], octets[3], octets[4], octets[5],
            );

            Ok(Request::Handshake { mac })
        }
        consts::REQ_KIND_POST_RESULTS => {
            let temperature = utils::deserialize_f32(bytes)?;
            let humidity = utils::next_byte(bytes)?;
            let air_pressure = utils::deserialize_optional(bytes, utils::deserialize_u16)?;

            Ok(Request::PostResults {
                temperature,
                humidity,
                air_pressure,
            })
        }
        consts::REQ_KIND_POST_STATS => {
            let battery = utils::deserialize_f32(bytes)?;
            let wifi_ssid = utils::deserialize_string(bytes)?;
            let wifi_rssi = utils::deserialize_i8(bytes)?;

            Ok(Request::PostStats {
                battery,
                wifi_ssid,
                wifi_rssi,
            })
        }
        consts::REQ_KIND_SEND_NOTIFICATION => {
            let content = utils::deserialize_string(bytes)?;
            Ok(Request::SendNotification(content))
        }
        consts::REQ_KIND_GET_SETTINGS => Ok(Request::GetSettings),
        consts::REQ_KIND_UPDATE_CHECK => {
            let version = utils::deserialize_version(bytes)?;
            Ok(Request::UpdateCheck(version))
        }
        consts::REQ_KIND_NEXT_UPDATE_CHUNK => {
            let amount = utils::deserialize_usize(bytes)?;
            Ok(Request::NextUpdateChunk(amount))
        }
        consts::REQ_KIND_REPORT_FWU => {
            let success = utils::deserialize_bool(bytes)?;
            Ok(Request::ReportFirmwareUpdate(success))
        }
        consts::REQ_KIND_BYE => Ok(Request::Bye),
        _ => Err(DeserializeError::IllegalRequestType(req_type)),
    }
}

/// Deserialize a response.
pub fn deserialize_response(
    bytes: &mut impl Iterator<Item = u8>,
) -> Result<Response, DeserializeError> {
    // get the response type
    let res_type = next_byte(bytes)?;

    match res_type {
        consts::RES_KIND_PONG => Ok(Response::Pong),
        consts::RES_KIND_OK => Ok(Response::Ok),
        consts::RES_KIND_REJECT => Ok(Response::Reject),
        consts::RES_KIND_INVALID_REQ => Ok(Response::InvalidRequest),
        consts::RES_KIND_RLE => Ok(Response::RateLimitExceeded),
        consts::RES_KIND_ISE => Ok(Response::InternalServerError),
        consts::RES_KIND_STALLING => Ok(Response::Stalling),
        consts::RES_KIND_FW_UTD => Ok(Response::FirmwareUpToDate),
        consts::RES_KIND_FW_UAVAIL => {
            let version = utils::deserialize_version(bytes)?;
            Ok(Response::UpdateAvailable(version))
        }
        consts::RES_KIND_FW_UPART => {
            let blob = utils::deserialize_blob(bytes)?;
            Ok(Response::UpdatePart(blob))
        }
        consts::RES_KIND_FW_UEND => Ok(Response::UpdateEnd),
        consts::RES_KIND_SETTINGS => {
            if bytes.next().unwrap_or_default() == 0 {
                return Ok(Response::Settings(None));
            }

            let battery_ignore = utils::deserialize_bool(bytes)?;
            let ota = utils::deserialize_bool(bytes)?;
            let sleep_time = utils::deserialize_u16(bytes)?;
            let sbop = utils::deserialize_bool(bytes)?;
            let mute_notifications = utils::deserialize_bool(bytes)?;

            Ok(Response::Settings(Some(NodeSettings {
                battery_ignore,
                ota,
                sleep_time,
                sbop,
                mute_notifications,
            })))
        }
        _ => Err(DeserializeError::IllegalResponseType(res_type)),
    }
}

/// Deserialize a message.
pub fn deserialize(bytes: &[u8]) -> Result<Message, DeserializeError> {
    let mut bytes = bytes.iter().copied();

    // get the message ID
    let msg_id = utils::deserialize_u32(&mut bytes)?;

    // get the message type (req/res)
    let msg_type = utils::next_byte(&mut bytes)?;

    match msg_type {
        consts::MSG_KIND_REQUEST => {
            let req = deserialize_request(&mut bytes)?;
            Ok(Message::new_request(req, msg_id))
        }
        consts::MSG_KIND_RESPONSE => {
            let res = deserialize_response(&mut bytes)?;
            Ok(Message::new_response(res, msg_id))
        }
        _ => Err(DeserializeError::IllegalMessageType(msg_type)),
    }
}
