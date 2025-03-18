use pwmp_msg::{response::Response, settings::NodeSettings, version::Version, Message};

macro_rules! generate_test {
    ($test_name: ident, $res: expr) => {
        #[test]
        fn $test_name() {
            let _ = Message::new_response($res, 0).serialize();
        }
    };
}

generate_test!(can_serialize_pong, Response::Pong);

generate_test!(can_serialize_ok, Response::Ok);

generate_test!(can_serialize_reject, Response::Reject);

generate_test!(can_serialize_fw_up_to_date, Response::FirmwareUpToDate);

generate_test!(
    can_serialize_update_available,
    Response::UpdateAvailable(Version::default())
);

generate_test!(
    can_serialize_update_part,
    Response::UpdatePart(b"blob".to_vec().into_boxed_slice())
);

generate_test!(can_serialize_update_end, Response::UpdateEnd);

generate_test!(can_serialize_empty_settings, Response::Settings(None));
generate_test!(
    can_serialize_settings,
    Response::Settings(Some(NodeSettings {
        battery_ignore: true,
        ota: true,
        sleep_time: 16,
        sbop: false,
        mute_notifications: true
    }))
);
