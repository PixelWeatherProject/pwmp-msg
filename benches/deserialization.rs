use criterion::{criterion_group, criterion_main, Criterion};
use pwmp_msg::{mac::Mac, request::Request, version::Version, Message};
use std::hint::black_box;

macro_rules! generate_benchmark {
    ($name: ident, $string_name: literal, $msg: expr) => {
        fn $name(c: &mut Criterion) {
            let serialized = Message::serialize($msg);
            c.bench_function($string_name, |b| {
                b.iter(|| Message::deserialize(black_box(&serialized)))
            });
        }
    };
}

generate_benchmark!(
    benchmark_request_ping_deserialization,
    "Message(Request::Ping)::deserialize",
    Message::new_request(Request::Ping, 55)
);

generate_benchmark!(
    benchmark_request_handshake_deserialization,
    "Message(Request::Handshake)::deserialize",
    Message::new_request(
        Request::Handshake {
            mac: Mac::new(0, 1, 2, 3, 4, 5)
        },
        55
    )
);

generate_benchmark!(
    benchmark_request_post_measurements_deserialization,
    "Message(Request::PostMeasurements)::deserialize",
    Message::new_request(
        Request::PostMeasurements {
            temperature: 45.78,
            humidity: 45,
            air_pressure: Some(65520),
            cpu_temp: 29.45,
            battery: 4.20,
            wifi_ssid: "Hello, World!".into(),
            wifi_rssi: -45,
        },
        55
    )
);

generate_benchmark!(
    benchmark_request_send_notification_deserialization,
    "Message(Request::SendNotification)::deserialize",
    Message::new_request(Request::SendNotification("Hello, World!".into()), 55)
);

generate_benchmark!(
    benchmark_request_get_settings_deserialization,
    "Message(Request::GetSettings)::deserialize",
    Message::new_request(Request::GetSettings, 55)
);

generate_benchmark!(
    benchmark_request_update_check_deserialization,
    "Message(Request::UpdateCheck)::deserialize",
    Message::new_request(Request::UpdateCheck(Version::new(5, 4, 7)), 55)
);

generate_benchmark!(
    benchmark_request_next_update_chunk_deserialization,
    "Message(Request::NextUpdateChunk)::deserialize",
    Message::new_request(Request::NextUpdateChunk(128_000_000), 55)
);

generate_benchmark!(
    benchmark_request_report_fw_update_deserialization,
    "Message(Request::ReportFirmwareUpdate)::deserialize",
    Message::new_request(Request::ReportFirmwareUpdate(true), 55)
);

generate_benchmark!(
    benchmark_request_bye_deserialization,
    "Message(Request::Bye)::deserialize",
    Message::new_request(Request::Bye, 55)
);

criterion_group!(
    benches,
    benchmark_request_ping_deserialization,
    benchmark_request_handshake_deserialization,
    benchmark_request_post_measurements_deserialization,
    benchmark_request_send_notification_deserialization,
    benchmark_request_get_settings_deserialization,
    benchmark_request_update_check_deserialization,
    benchmark_request_next_update_chunk_deserialization,
    benchmark_request_report_fw_update_deserialization,
    benchmark_request_bye_deserialization
);
criterion_main!(benches);
