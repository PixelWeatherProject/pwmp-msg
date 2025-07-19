//! A Serialization and deserialization implementation for PWMP.

use super::{request::Request, response::Response, Message, MessageContent};
use crate::{aliases::AirPressure, mac::Mac, settings::NodeSettings, MsgId};

pub mod consts;
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
            buffer.extend_from_slice(&[mac[0], mac[1], mac[2], mac[3], mac[4], mac[5], mac[6]]);
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
pub fn deserialize_request(bytes: &[u8]) -> Option<Request> {
    let mut bytes = bytes.iter().copied();

    // get the request type
    let req_type = bytes.next()?;

    match req_type {
        consts::REQ_KIND_PING => Some(Request::Ping),
        consts::REQ_KIND_HANDSHAKE => {
            let mac = Mac::new(
                bytes.next()?,
                bytes.next()?,
                bytes.next()?,
                bytes.next()?,
                bytes.next()?,
                bytes.next()?,
            );

            Some(Request::Handshake { mac })
        }
        consts::REQ_KIND_POST_RESULTS => {
            let temperature_bytes = std::array::from_fn(|_| bytes.next().unwrap());
            let temperature = f32::from_ne_bytes(temperature_bytes);
            let humidity = bytes.next()?;
            let air_pressure = if bytes.next()? == 0 {
                None
            } else {
                let air_pressure_bytes = std::array::from_fn(|_| bytes.next().unwrap());
                Some(AirPressure::from_ne_bytes(air_pressure_bytes))
            };

            Some(Request::PostResults {
                temperature,
                humidity,
                air_pressure,
            })
        }
        consts::REQ_KIND_POST_STATS => {
            let battery = utils::deserialize_f32(&mut bytes);
            let wifi_ssid = utils::deserialize_string(&mut bytes);
            let wifi_rssi = utils::deserialize_i8(&mut bytes);

            Some(Request::PostStats {
                battery,
                wifi_ssid,
                wifi_rssi,
            })
        }
        consts::REQ_KIND_SEND_NOTIFICATION => {
            let content = utils::deserialize_string(&mut bytes);
            Some(Request::SendNotification(content))
        }
        consts::REQ_KIND_GET_SETTINGS => Some(Request::GetSettings),
        consts::REQ_KIND_UPDATE_CHECK => {
            let version = utils::deserialize_version(&mut bytes);
            Some(Request::UpdateCheck(version))
        }
        consts::REQ_KIND_NEXT_UPDATE_CHUNK => {
            let amount = utils::deserialize_usize(&mut bytes);
            Some(Request::NextUpdateChunk(amount))
        }
        consts::REQ_KIND_REPORT_FWU => {
            let success = utils::deserialize_bool(&mut bytes);
            Some(Request::ReportFirmwareUpdate(success))
        }
        consts::REQ_KIND_BYE => Some(Request::Bye),
        _ => None,
    }
}

/// Deserialize a response.
pub fn deserialize_response(bytes: &[u8]) -> Option<Response> {
    let mut bytes = bytes.iter().copied();

    // get the response type
    let req_type = bytes.next()?;

    match req_type {
        consts::RES_KIND_PONG => Some(Response::Pong),
        consts::RES_KIND_OK => Some(Response::Ok),
        consts::RES_KIND_REJECT => Some(Response::Reject),
        consts::RES_KIND_INVALID_REQ => Some(Response::InvalidRequest),
        consts::RES_KIND_RLE => Some(Response::RateLimitExceeded),
        consts::RES_KIND_ISE => Some(Response::InternalServerError),
        consts::RES_KIND_STALLING => Some(Response::Stalling),
        consts::RES_KIND_FW_UTD => Some(Response::FirmwareUpToDate),
        consts::RES_KIND_FW_UAVAIL => {
            let version = utils::deserialize_version(&mut bytes);
            Some(Response::UpdateAvailable(version))
        }
        consts::RES_KIND_FW_UPART => {
            let blob = utils::deserialize_blob(&mut bytes);
            Some(Response::UpdatePart(blob.into_boxed_slice()))
        }
        consts::RES_KIND_FW_UEND => Some(Response::UpdateEnd),
        consts::RES_KIND_SETTINGS => {
            if bytes.next().unwrap_or_default() == 0 {
                return Some(Response::Settings(None));
            }

            let battery_ignore = utils::deserialize_bool(&mut bytes);
            let ota = utils::deserialize_bool(&mut bytes);
            let sleep_time = utils::deserialize_u16(&mut bytes);
            let sbop = utils::deserialize_bool(&mut bytes);
            let mute_notifications = utils::deserialize_bool(&mut bytes);

            Some(Response::Settings(Some(NodeSettings {
                battery_ignore,
                ota,
                sleep_time,
                sbop,
                mute_notifications,
            })))
        }
        _ => None,
    }
}

/// Deserialize a message.
pub fn deserialize(bytes: &[u8]) -> Option<Message> {
    // get the message ID
    let msg_id = MsgId::from_ne_bytes(bytes[..4].try_into().unwrap());

    // get the message type (req/res)
    let msg_type = bytes[0];

    match msg_type {
        consts::MSG_KIND_REQUEST => {
            let req = deserialize_request(&bytes[1..])?;
            Some(Message::new_request(req, msg_id))
        }
        consts::MSG_KIND_RESPONSE => {
            let res = deserialize_response(&bytes[1..])?;
            Some(Message::new_response(res, msg_id))
        }
        _ => None,
    }
}
