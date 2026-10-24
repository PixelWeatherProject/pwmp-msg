use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use pwmp_msg::{mac::Mac, request::Request, version::Version, Message};
use std::hint::black_box;

macro_rules! generate_benchmark {
    ($name: ident, $string_name: literal, $msg: expr) => {
        fn $name(c: &mut Criterion) {
            c.bench_function($string_name, |b| {
                b.iter_batched(
                    || $msg,
                    |msg| Message::serialize(black_box(msg)),
                    BatchSize::SmallInput,
                )
            });
        }
    };
}

generate_benchmark!(
    benchmark_request_ping_serialization,
    "Message(Request::Ping)::serialize",
    Message::new_request(Request::Ping, 55)
);

generate_benchmark!(
    benchmark_request_handshake_serialization,
    "Message(Request::Handshake)::serialize",
    Message::new_request(
        Request::Handshake {
            mac: Mac::new(0, 1, 2, 3, 4, 5)
        },
        55
    )
);

generate_benchmark!(
    benchmark_request_post_measurements_serialization,
    "Message(Request::PostMeasurements)::serialize",
    Message::new_request(
        Request::PostMeasurements {
            temperature: 45.78,
            humidity: 45,
            air_pressure: Some(65520),
            cpu_temp: 27.12,
            battery: 4.20,
            wifi_ssid: "Hello, World!".into(),
            wifi_rssi: -45,
        },
        55
    )
);

generate_benchmark!(
    benchmark_request_send_notification_serialization,
    "Message(Request::SendNotification)::serialize",
    Message::new_request(Request::SendNotification("Hello, World!".into()), 55)
);

generate_benchmark!(
    benchmark_request_get_settings_serialization,
    "Message(Request::GetSettings)::serialize",
    Message::new_request(Request::GetSettings, 55)
);

generate_benchmark!(
    benchmark_request_update_check_serialization,
    "Message(Request::UpdateCheck)::serialize",
    Message::new_request(Request::UpdateCheck(Version::new(5, 4, 7)), 55)
);

generate_benchmark!(
    benchmark_request_next_update_chunk_serialization,
    "Message(Request::NextUpdateChunk)::serialize",
    Message::new_request(Request::NextUpdateChunk(128_000_000), 55)
);

generate_benchmark!(
    benchmark_request_report_fw_update_serialization,
    "Message(Request::ReportFirmwareUpdate)::serialize",
    Message::new_request(Request::ReportFirmwareUpdate(true), 55)
);

generate_benchmark!(
    benchmark_request_bye_serialization,
    "Message(Request::Bye)::serialize",
    Message::new_request(Request::Bye, 55)
);

criterion_group!(
    benches,
    benchmark_request_ping_serialization,
    benchmark_request_handshake_serialization,
    benchmark_request_post_measurements_serialization,
    benchmark_request_send_notification_serialization,
    benchmark_request_get_settings_serialization,
    benchmark_request_update_check_serialization,
    benchmark_request_next_update_chunk_serialization,
    benchmark_request_report_fw_update_serialization,
    benchmark_request_bye_serialization
);
criterion_main!(benches);
