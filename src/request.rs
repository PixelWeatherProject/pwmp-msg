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
    /// Message ID for [`Request::Ping`].
    pub const MSG_ID_PING: u8 = 0;
    /// Message ID for [`Request::Handshake`].
    pub const MSG_ID_HANDSHAKE: u8 = 1;
    /// Message ID for [`Request::PostResults`].
    pub const MSG_ID_POST_RESULTS: u8 = 2;
    /// Message ID for [`Request::PostStats`].
    pub const MSG_ID_POST_STATS: u8 = 3;
    /// Message ID for [`Request::SendNotification`].
    pub const MSG_ID_SEND_NOTIFICATION: u8 = 4;
    /// Message ID for [`Request::GetSettings`].
    pub const MSG_ID_GET_SETTINGS: u8 = 5;
    /// Message ID for [`Request::UpdateCheck`].
    pub const MSG_ID_UPDATE_CHECK: u8 = 6;
    /// Message ID for [`Request::NextUpdateChunk`].
    pub const MSG_ID_NEXT_UPDATE_CHUNK: u8 = 7;
    /// Message ID for [`Request::ReportFirmwareUpdate`].
    pub const MSG_ID_REPORT_FIRMWARE_UPDATE: u8 = 8;
    /// Message ID for [`Request::Bye`].
    pub const MSG_ID_BYE: u8 = 9;
}
