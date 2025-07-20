use criterion::{criterion_group, criterion_main, Criterion};
use pwmp_msg::{mac::Mac, request::Request, version::Version, Message};
use std::hint::black_box;

macro_rules! bb {
    ($e: expr) => {
        black_box($e)
    };
}

macro_rules! generate_benchmark {
    ($name: ident, $string_name: literal, $msg: expr) => {
        fn $name(c: &mut Criterion) {
            let mut buffer = Vec::with_capacity(128);
            c.bench_function($string_name, |b| {
                b.iter(|| Message::serialize_into(bb!($msg), &mut buffer))
            });
        }
    };
}

generate_benchmark!(
    benchmark_request_ping_serialization,
    "Message(Request::Ping)::serialize",
    Message::new_request(bb!(Request::Ping), bb!(55))
);

generate_benchmark!(
    benchmark_request_handshake_serialization,
    "Message(Request::Handshake)::serialize",
    Message::new_request(
        bb!(Request::Handshake {
            mac: bb!(Mac::new(bb!(0), bb!(1), bb!(2), bb!(3), bb!(4), bb!(5)))
        }),
        bb!(55)
    )
);

generate_benchmark!(
    benchmark_request_post_results_serialization,
    "Message(Request::PostResults)::serialize",
    Message::new_request(
        bb!(Request::PostResults {
            temperature: bb!(45.78),
            humidity: bb!(45),
            air_pressure: bb!(Some(bb!(65520))),
        }),
        bb!(55)
    )
);

generate_benchmark!(
    benchmark_request_post_stats_serialization,
    "Message(Request::PostStats)::serialize",
    Message::new_request(
        bb!(Request::PostStats {
            battery: bb!(4.20),
            wifi_ssid: bb!("Hello, World!".to_string().into_boxed_str()),
            wifi_rssi: bb!(-45),
        }),
        bb!(55)
    )
);

generate_benchmark!(
    benchmark_request_send_notification_serialization,
    "Message(Request::SendNotification)::serialize",
    Message::new_request(
        bb!(Request::SendNotification(bb!("Hello, World!"
            .to_string()
            .into_boxed_str()))),
        bb!(55)
    )
);

generate_benchmark!(
    benchmark_request_get_settings_serialization,
    "Message(Request::GetSettings)::serialize",
    Message::new_request(bb!(Request::GetSettings), bb!(55))
);

generate_benchmark!(
    benchmark_request_update_check_serialization,
    "Message(Request::UpdateCheck)::serialize",
    Message::new_request(
        bb!(Request::UpdateCheck(bb!(Version::new(
            bb!(5),
            bb!(4),
            bb!(7)
        )))),
        bb!(55)
    )
);

generate_benchmark!(
    benchmark_request_next_update_chunk_serialization,
    "Message(Request::NextUpdateChunk)::serialize",
    Message::new_request(bb!(Request::NextUpdateChunk(bb!(128_000_000))), bb!(55))
);

generate_benchmark!(
    benchmark_request_report_fw_update_serialization,
    "Message(Request::ReportFirmwareUpdate)::serialize",
    Message::new_request(bb!(Request::ReportFirmwareUpdate(bb!(true))), bb!(55))
);

generate_benchmark!(
    benchmark_request_bye_serialization,
    "Message(Request::Bye)::serialize",
    Message::new_request(bb!(Request::Bye), bb!(55))
);

criterion_group!(
    benches,
    benchmark_request_ping_serialization,
    benchmark_request_handshake_serialization,
    benchmark_request_post_results_serialization,
    benchmark_request_post_stats_serialization,
    benchmark_request_send_notification_serialization,
    benchmark_request_get_settings_serialization,
    benchmark_request_update_check_serialization,
    benchmark_request_next_update_chunk_serialization,
    benchmark_request_report_fw_update_serialization,
    benchmark_request_bye_serialization
);
criterion_main!(benches);
