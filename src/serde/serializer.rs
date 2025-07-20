//! Definition of the `Serializable` trait and it's implementations.

use super::consts;
use crate::{
    mac::Mac, request::Request, response::Response, settings::NodeSettings, version::Version,
};

/// An object that is serializable as a sequence of bytes.
pub trait Serializable {
    /// Serialize the object into a vector of bytes.
    fn serialize(&self, buffer: &mut Vec<u8>);
}

/// Generate an implementation of the `Serializable` trait for a primitive type.
macro_rules! impl_primitive_serialization {
    ($t: ty) => {
        impl Serializable for $t {
            fn serialize(&self, buffer: &mut Vec<u8>) {
                buffer.extend_from_slice(&self.to_ne_bytes());
            }
        }
    };
}

// signed integers
impl_primitive_serialization!(i8);

// unsigned integers
impl_primitive_serialization!(u8);
impl_primitive_serialization!(u16);
impl_primitive_serialization!(u32);
impl_primitive_serialization!(usize);

// floats
impl_primitive_serialization!(f32);

impl Serializable for Mac {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        buffer.push(self[0]);
        buffer.push(self[1]);
        buffer.push(self[2]);
        buffer.push(self[3]);
        buffer.push(self[4]);
        buffer.push(self[5]);
    }
}

impl Serializable for Version {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.major());
        buffer.push(self.middle());
        buffer.push(self.minor());
    }
}

impl Serializable for NodeSettings {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        self.battery_ignore.serialize(buffer);
        self.ota.serialize(buffer);
        self.sleep_time.serialize(buffer);
        self.sbop.serialize(buffer);
        self.mute_notifications.serialize(buffer);
    }
}

// booleans
impl Serializable for bool {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        buffer.push(u8::from(*self));
    }
}

// byte array
impl Serializable for &[u8] {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.len().to_ne_bytes());
        buffer.extend_from_slice(self);
    }
}

// strings
impl Serializable for &str {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        self.as_bytes().serialize(buffer);
    }
}

// optionals
impl<T: Serializable> Serializable for Option<T> {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        match self {
            None => {
                buffer.push(consts::OPTIONAL_EMPTY);
            }
            Some(val) => {
                buffer.push(consts::OPTIONAL_EXIST);
                val.serialize(buffer);
            }
        }
    }
}

impl Serializable for Request {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        // push request type
        self.type_id().serialize(buffer);

        match self {
            Request::Ping => { /* the above push already pushes consts::REQ_KIND_PING */ }
            Request::Handshake { mac } => {
                mac.serialize(buffer);
            }
            Request::PostResults {
                temperature,
                humidity,
                air_pressure,
            } => {
                temperature.serialize(buffer);
                humidity.serialize(buffer);
                air_pressure.serialize(buffer);
            }
            Request::PostStats {
                battery,
                wifi_ssid,
                wifi_rssi,
            } => {
                battery.serialize(buffer);
                wifi_ssid.as_ref().serialize(buffer);
                wifi_rssi.serialize(buffer);
            }
            Request::SendNotification(content) => {
                content.as_ref().serialize(buffer);
            }
            Request::GetSettings => {}
            Request::UpdateCheck(current_ver) => {
                current_ver.serialize(buffer);
            }
            Request::NextUpdateChunk(size) => {
                size.serialize(buffer);
            }
            Request::ReportFirmwareUpdate(good) => {
                good.serialize(buffer);
            }
            Request::Bye => { /* the above push already pushes consts::REQ_KIND_BYE */ }
        }
    }
}

impl Serializable for Response {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        // push the response type
        self.type_id().serialize(buffer);

        match self {
            Response::Pong => {}
            Response::Ok => {}
            Response::Reject => {}
            Response::InvalidRequest => {}
            Response::RateLimitExceeded => {}
            Response::InternalServerError => {}
            Response::Stalling => {}
            Response::FirmwareUpToDate => {}
            Response::UpdateAvailable(new_version) => {
                new_version.serialize(buffer);
            }
            Response::UpdatePart(blob) => {
                blob.as_ref().serialize(buffer);
            }
            Response::UpdateEnd => {}
            Response::Settings(settings) => {
                settings.serialize(buffer);
            }
        }
    }
}
