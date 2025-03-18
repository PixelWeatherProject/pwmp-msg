use pwmp_msg::{mac::Mac, request::Request, version::Version, Decimal, Message};

#[test]
fn can_serialize_ping() {
    let _ = Message::new_request(Request::Ping, 0).serialize();
}

#[test]
fn can_serialize_hello() {
    let _ = Message::new_request(
        Request::Hello {
            mac: Mac::new(0, 1, 2, 3, 4, 5),
        },
        u32::MAX,
    )
    .serialize();
}

#[test]
fn can_serialize_post_results() {
    let _ = Message::new_request(
        Request::PostResults {
            temperature: Decimal::default(),
            humidity: 50,
            air_pressure: Some(u16::MAX),
        },
        u32::MIN,
    )
    .serialize();
}

#[test]
fn can_serialize_post_stats() {
    let _ = Message::new_request(
        Request::PostStats {
            battery: Decimal::default(),
            wifi_ssid: "ABC 123".into(),
            wifi_rssi: -15,
        },
        6,
    )
    .serialize();
}

#[test]
fn can_serialize_send_notification() {
    let _ = Message::new_request(Request::SendNotification("Hello, World!".into()), 1).serialize();
}

#[test]
fn can_serialize_get_settings() {
    let _ = Message::new_request(Request::GetSettings, 0).serialize();
}

#[test]
fn can_serialize_update_check() {
    let _ = Message::new_request(Request::UpdateCheck(Version::new(1, 0, 1)), 0).serialize();
}

#[test]
fn can_serialize_next_update_chunk() {
    let _ = Message::new_request(Request::NextUpdateChunk(256), 0).serialize();
}

#[test]
fn can_serialize_report_fw_update() {
    let _ = Message::new_request(Request::ReportFirmwareUpdate(true), 0).serialize();
    let _ = Message::new_request(Request::ReportFirmwareUpdate(false), 0).serialize();
}
