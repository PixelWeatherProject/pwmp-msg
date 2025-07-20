//! Definition of the `Deserializable` trait and it's implementations.

use super::{consts, utils, DeserializeError};
use crate::{
    aliases::{AirPressure, BatteryVoltage, Humidity, Rssi, Temperature},
    mac::Mac,
    request::Request,
    response::Response,
    settings::NodeSettings,
    version::Version,
};

/// An object that is deserializable from a sequence of bytes.
pub trait Deserializable: Sized {
    /// Deserialize the object from an interator of bytes.
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = u8>;
}

/// Generate an implementation of the `Deserializable` trait for a primitive type.
macro_rules! impl_primitive_deserialization {
    ($t: ty) => {
        impl Deserializable for $t {
            fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
            where
                I: Iterator<Item = u8>,
            {
                let value_bytes = utils::next_bytes(bytes)?;
                Ok(Self::from_ne_bytes(value_bytes))
            }
        }
    };
}

impl Deserializable for u8 {
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = Self>,
    {
        utils::next_byte(bytes)
    }
}

impl Deserializable for i8 {
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = u8>,
    {
        let value_byte = utils::next_byte(bytes)?;
        Ok(Self::from_ne_bytes([value_byte]))
    }
}

// unsigned integers
impl_primitive_deserialization!(u16);
impl_primitive_deserialization!(u32);
impl_primitive_deserialization!(usize);

// floats
impl_primitive_deserialization!(f32);

impl Deserializable for bool {
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = u8>,
    {
        match utils::next_byte(bytes)? {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(DeserializeError::IllegalBoolean(other)),
        }
    }
}

impl Deserializable for Mac {
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = u8>,
    {
        let octets = utils::next_bytes(bytes)?;
        Ok(Self::from(octets))
    }
}

impl Deserializable for Version {
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = u8>,
    {
        let values = utils::next_bytes(bytes)?;
        Ok(Self::from(values))
    }
}

impl Deserializable for NodeSettings {
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = u8>,
    {
        let battery_ignore = bool::deserialize(bytes)?;
        let ota = bool::deserialize(bytes)?;
        let sleep_time = u16::deserialize(bytes)?;
        let sbop = bool::deserialize(bytes)?;
        let mute_notifications = bool::deserialize(bytes)?;

        Ok(Self {
            battery_ignore,
            ota,
            sleep_time,
            sbop,
            mute_notifications,
        })
    }
}

// byte array
impl Deserializable for Box<[u8]> {
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = u8>,
    {
        let len = usize::deserialize(bytes)?;
        let blob: Self = bytes.by_ref().take(len).collect();

        if blob.len() != len {
            return Err(DeserializeError::ExpectedMoreBytes {
                expected: len,
                got: blob.len(),
            });
        }

        Ok(blob)
    }
}

// strings
impl Deserializable for Box<str> {
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = u8>,
    {
        let blob = Box::<[u8]>::deserialize(bytes)?;
        let string = String::from_utf8(blob.into_vec())?;
        Ok(string.into_boxed_str())
    }
}

// optionals
impl<T: Deserializable> Deserializable for Option<T> {
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = u8>,
    {
        if !bool::deserialize(bytes)? {
            return Ok(None);
        }

        Ok(Some(T::deserialize(bytes)?))
    }
}

impl Deserializable for Request {
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = u8>,
    {
        // get the request type
        let req_type = utils::next_byte(bytes)?;

        match req_type {
            consts::REQ_KIND_PING => Ok(Self::Ping),
            consts::REQ_KIND_HANDSHAKE => {
                let mac = Mac::deserialize(bytes)?;
                Ok(Self::Handshake { mac })
            }
            consts::REQ_KIND_POST_RESULTS => Ok(Self::PostResults {
                temperature: Temperature::deserialize(bytes)?,
                humidity: Humidity::deserialize(bytes)?,
                air_pressure: Option::<AirPressure>::deserialize(bytes)?,
            }),
            consts::REQ_KIND_POST_STATS => Ok(Self::PostStats {
                battery: BatteryVoltage::deserialize(bytes)?,
                wifi_ssid: Box::<str>::deserialize(bytes)?,
                wifi_rssi: Rssi::deserialize(bytes)?,
            }),
            consts::REQ_KIND_SEND_NOTIFICATION => {
                let content = Box::<str>::deserialize(bytes)?;
                Ok(Self::SendNotification(content))
            }
            consts::REQ_KIND_GET_SETTINGS => Ok(Self::GetSettings),
            consts::REQ_KIND_UPDATE_CHECK => {
                let version = Version::deserialize(bytes)?;
                Ok(Self::UpdateCheck(version))
            }
            consts::REQ_KIND_NEXT_UPDATE_CHUNK => {
                let amount = usize::deserialize(bytes)?;
                Ok(Self::NextUpdateChunk(amount))
            }
            consts::REQ_KIND_REPORT_FWU => {
                let success = bool::deserialize(bytes)?;
                Ok(Self::ReportFirmwareUpdate(success))
            }
            consts::REQ_KIND_BYE => Ok(Self::Bye),
            _ => Err(DeserializeError::IllegalRequestType(req_type)),
        }
    }
}

impl Deserializable for Response {
    fn deserialize<I>(bytes: &mut I) -> Result<Self, DeserializeError>
    where
        I: Iterator<Item = u8>,
    {
        // get the response type
        let res_type = utils::next_byte(bytes)?;

        match res_type {
            consts::RES_KIND_PONG => Ok(Self::Pong),
            consts::RES_KIND_OK => Ok(Self::Ok),
            consts::RES_KIND_REJECT => Ok(Self::Reject),
            consts::RES_KIND_INVALID_REQ => Ok(Self::InvalidRequest),
            consts::RES_KIND_RLE => Ok(Self::RateLimitExceeded),
            consts::RES_KIND_ISE => Ok(Self::InternalServerError),
            consts::RES_KIND_STALLING => Ok(Self::Stalling),
            consts::RES_KIND_FW_UTD => Ok(Self::FirmwareUpToDate),
            consts::RES_KIND_FW_UAVAIL => {
                let version = Version::deserialize(bytes)?;
                Ok(Self::UpdateAvailable(version))
            }
            consts::RES_KIND_FW_UPART => {
                let blob = Box::<[u8]>::deserialize(bytes)?;
                Ok(Self::UpdatePart(blob))
            }
            consts::RES_KIND_FW_UEND => Ok(Self::UpdateEnd),
            consts::RES_KIND_SETTINGS => {
                let settings = Option::<NodeSettings>::deserialize(bytes)?;
                Ok(Self::Settings(settings))
            }
            _ => Err(DeserializeError::IllegalResponseType(res_type)),
        }
    }
}
