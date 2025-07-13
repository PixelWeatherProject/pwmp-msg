//! A Serialization and deserialization implementation for PWMP.

use super::{request::Request, response::Response, Message, MessageContent};

mod consts;

/// Serialize a message.
pub fn serialize(msg: Message) -> Box<[u8]> {
    let mut buffer = Vec::with_capacity(128);

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

            serialize_optional(
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
            serialize_blob(wifi_ssid.as_bytes(), buffer);
            buffer.extend_from_slice(&wifi_rssi.to_ne_bytes());
        }
        Request::SendNotification(content) => {
            serialize_blob(content.as_bytes(), buffer);
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
            serialize_optional(
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

/// Serialize a blob.
fn serialize_blob(val: &[u8], buffer: &mut Vec<u8>) {
    buffer.reserve(size_of::<usize>() + val.len());
    buffer.extend_from_slice(&val.len().to_ne_bytes());
    buffer.extend_from_slice(val);
}

/// Serialize an optinal.
fn serialize_optional<T, F>(val: Option<T>, value_serializer: F, buffer: &mut Vec<u8>)
where
    F: FnOnce(T, &mut Vec<u8>),
{
    match val {
        Some(inner) => {
            buffer.push(consts::OPTIONAL_EXIST);
            value_serializer(inner, buffer);
        }
        None => {
            buffer.push(consts::OPTIONAL_EMPTY);
        }
    }
}

/// Deserialize a message.
pub fn deserialize(_bytes: &[u8]) -> Option<Message> {
    todo!()
}
