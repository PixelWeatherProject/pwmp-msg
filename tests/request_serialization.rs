use pwmp_msg::{mac::Mac, request::Request, version::Version, Decimal, Message};

macro_rules! generate_test {
    ($test_name: ident, $req: expr) => {
        #[test]
        fn $test_name() {
            let _ = Message::new_request($req, 0).serialize();
        }
    };
}

generate_test!(can_serialize_ping, Request::Ping);

generate_test!(
    can_serialize_hello,
    Request::Hello {
        mac: Mac::new(0, 1, 2, 3, 4, 5),
    }
);

generate_test!(
    can_serialize_post_results,
    Request::PostResults {
        temperature: Decimal::default(),
        humidity: 50,
        air_pressure: Some(u16::MAX),
    }
);

generate_test!(
    can_serialize_post_stats,
    Request::PostStats {
        battery: Decimal::default(),
        wifi_ssid: "ABC 123".into(),
        wifi_rssi: -15,
    }
);

generate_test!(
    can_serialize_send_notification,
    Request::SendNotification("Hello, World!".into())
);

generate_test!(can_serialize_get_settings, Request::GetSettings);

generate_test!(
    can_serialize_update_check,
    Request::UpdateCheck(Version::new(1, 0, 1))
);

generate_test!(can_serialize_update_chunk, Request::NextUpdateChunk(256));

generate_test!(
    can_serialize_report_pos_fw_update,
    Request::ReportFirmwareUpdate(true)
);

generate_test!(
    can_serialize_report_neg_fw_update,
    Request::ReportFirmwareUpdate(false)
);
