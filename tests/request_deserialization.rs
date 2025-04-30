use pwmp_msg::{mac::Mac, request::Request, version::Version, Message};

macro_rules! generate_test {
    ($test_name: ident, $req: expr) => {
        #[test]
        fn $test_name() {
            let request: Request = $req;
            let original_message = Message::new_request(request, 1);
            let serialized = original_message.clone().serialize();
            let deserialized = Message::deserialize(&serialized).unwrap();

            assert_eq!(original_message, deserialized);
        }
    };
}

generate_test!(can_deserialize_ping, Request::Ping);

generate_test!(
    can_deserialize_hello,
    Request::Handshake {
        mac: Mac::new(0, 1, 2, 3, 4, 5),
    }
);

generate_test!(
    can_deserialize_post_results,
    Request::PostResults {
        temperature: f32::default(),
        humidity: 50,
        air_pressure: Some(u16::MAX),
    }
);

generate_test!(
    can_deserialize_post_stats,
    Request::PostStats {
        battery: f32::default(),
        wifi_ssid: "ABC 123".into(),
        wifi_rssi: -15,
    }
);

generate_test!(
    can_deserialize_send_notification,
    Request::SendNotification("Hello, World!".into())
);

generate_test!(can_deserialize_get_settings, Request::GetSettings);

generate_test!(
    can_deserialize_update_check,
    Request::UpdateCheck(Version::new(1, 0, 1))
);

generate_test!(can_deserialize_update_chunk, Request::NextUpdateChunk(256));

generate_test!(
    can_deserialize_report_pos_fw_update,
    Request::ReportFirmwareUpdate(true)
);

generate_test!(
    can_deserialize_report_neg_fw_update,
    Request::ReportFirmwareUpdate(false)
);
