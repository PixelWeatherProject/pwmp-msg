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
            Self::Ping | Self::GetSettings | Self::Bye => { /* no additional data to serialize */ }
            Self::Handshake { mac } => {
                mac.serialize(buffer);
            }
            Self::PostResults {
                temperature,
                humidity,
                air_pressure,
            } => {
                temperature.serialize(buffer);
                humidity.serialize(buffer);
                air_pressure.serialize(buffer);
            }
            Self::PostStats {
                battery,
                wifi_ssid,
                wifi_rssi,
            } => {
                battery.serialize(buffer);
                wifi_ssid.as_ref().serialize(buffer);
                wifi_rssi.serialize(buffer);
            }
            Self::SendNotification(content) => {
                content.as_ref().serialize(buffer);
            }
            Self::UpdateCheck(current_ver) => {
                current_ver.serialize(buffer);
            }
            Self::NextUpdateChunk(size) => {
                size.serialize(buffer);
            }
            Self::ReportFirmwareUpdate(good) => {
                good.serialize(buffer);
            }
        }
    }
}

impl Serializable for Response {
    fn serialize(&self, buffer: &mut Vec<u8>) {
        // push the response type
        self.type_id().serialize(buffer);

        match self {
            Self::Pong
            | Self::Ok
            | Self::Reject
            | Self::InvalidRequest
            | Self::RateLimitExceeded
            | Self::InternalServerError
            | Self::Stalling
            | Self::FirmwareUpToDate
            | Self::UpdateEnd => { /* no additional data to serialize */ }
            Self::UpdateAvailable(new_version) => {
                new_version.serialize(buffer);
            }
            Self::UpdatePart(blob) => {
                blob.as_ref().serialize(buffer);
            }
            Self::Settings(settings) => {
                settings.serialize(buffer);
            }
        }
    }
}
