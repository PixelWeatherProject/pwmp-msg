//! Contains the definition if a request message.

use crate::{
    aliases::{AirPressure, BatteryVoltage, Humidity, Rssi, Temperature},
    mac::Mac,
    version::Version,
};

/// A request message used by nodes to ask the PWMP server to perform an operation.
#[derive(Debug, PartialEq, Clone)]
pub enum Request {
    /// Used to check if the server is alive.
    Ping,

    /// Ask to server to authorize the node using it's MAC address.
    Handshake {
        #[allow(clippy::doc_markdown)]
        /// The node's MAC address. This address should be that of the WiFi interface.
        mac: Mac,
    },

    /// Post measurement results to the database.
    PostResults {
        /// Temperature
        temperature: Temperature,
        /// Humidity
        humidity: Humidity,
        /// Air pressure *(if supported by the node)*
        air_pressure: Option<AirPressure>,
    },

    /// Post node statistics to the database.
    PostStats {
        /// Node's battery voltage
        battery: BatteryVoltage,
        /// ESSID of the wireless network
        wifi_ssid: Box<str>,
        /// RSSI *(signal quality)* of the connection to the wireless network in dBm *(decibel meters)*.
        /// Higher values (closer to 0) mean a better quality.
        wifi_rssi: Rssi,
    },

    /// Store a notification in the database. These can be read by other applications.
    ///
    /// Notifications are usually used to inform about low battery status.
    SendNotification(Box<str>),

    /// Retrieve the node's settings from the database.
    GetSettings,

    /// Check for a firmware update.
    /// This will also cache the update on the server.
    UpdateCheck(Version),

    /// Request a part of a firmware upgrade. The parameter is the maximum chunk size that shall be received.
    ///
    /// **The client must request an update check first before sending this request.**
    NextUpdateChunk(usize),

    /// Report back about the updated firmware version.
    /// The parameter means whether this new firmware is working, or was bad, and the node has rolled back to a previous version.
    ReportFirmwareUpdate(bool),

    /// Tell the server that the session is over and the node will disconnect.
    Bye,
}

impl Request {
    /// Return the type ID.
    pub(crate) const fn type_id(&self) -> u8 {
        match self {
            Self::Ping => crate::serde::consts::REQ_KIND_PING,
            Self::Handshake { .. } => crate::serde::consts::REQ_KIND_HANDSHAKE,
            Self::PostResults { .. } => crate::serde::consts::REQ_KIND_POST_RESULTS,
            Self::PostStats { .. } => crate::serde::consts::REQ_KIND_POST_STATS,
            Self::SendNotification(..) => crate::serde::consts::REQ_KIND_SEND_NOTIFICATION,
            Self::GetSettings => crate::serde::consts::REQ_KIND_GET_SETTINGS,
            Self::UpdateCheck(..) => crate::serde::consts::REQ_KIND_UPDATE_CHECK,
            Self::NextUpdateChunk(..) => crate::serde::consts::REQ_KIND_NEXT_UPDATE_CHUNK,
            Self::ReportFirmwareUpdate(..) => crate::serde::consts::REQ_KIND_REPORT_FWU,
            Self::Bye => crate::serde::consts::REQ_KIND_BYE,
        }
    }
}
