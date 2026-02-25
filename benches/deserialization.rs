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
            let serialized = Message::serialize(bb!($msg));
            c.bench_function($string_name, |b| {
                b.iter(|| Message::deserialize(&serialized))
            });
        }
    };
}

generate_benchmark!(
    benchmark_request_ping_deserialization,
    "Message(Request::Ping)::deserialize",
    Message::new_request(bb!(Request::Ping), bb!(55))
);

generate_benchmark!(
    benchmark_request_handshake_deserialization,
    "Message(Request::Handshake)::deserialize",
    Message::new_request(
        bb!(Request::Handshake {
            mac: bb!(Mac::new(bb!(0), bb!(1), bb!(2), bb!(3), bb!(4), bb!(5)))
        }),
        bb!(55)
    )
);

generate_benchmark!(
    benchmark_request_post_results_deserialization,
    "Message(Request::PostResults)::deserialize",
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
    benchmark_request_post_stats_deserialization,
    "Message(Request::PostStats)::deserialize",
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
    benchmark_request_send_notification_deserialization,
    "Message(Request::SendNotification)::deserialize",
    Message::new_request(
        bb!(Request::SendNotification(bb!("Hello, World!"
            .to_string()
            .into_boxed_str()))),
        bb!(55)
    )
);

generate_benchmark!(
    benchmark_request_get_settings_deserialization,
    "Message(Request::GetSettings)::deserialize",
    Message::new_request(bb!(Request::GetSettings), bb!(55))
);

generate_benchmark!(
    benchmark_request_update_check_deserialization,
    "Message(Request::UpdateCheck)::deserialize",
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
    benchmark_request_next_update_chunk_deserialization,
    "Message(Request::NextUpdateChunk)::deserialize",
    Message::new_request(bb!(Request::NextUpdateChunk(bb!(128_000_000))), bb!(55))
);

generate_benchmark!(
    benchmark_request_report_fw_update_deserialization,
    "Message(Request::ReportFirmwareUpdate)::deserialize",
    Message::new_request(bb!(Request::ReportFirmwareUpdate(bb!(true))), bb!(55))
);

generate_benchmark!(
    benchmark_request_bye_deserialization,
    "Message(Request::Bye)::deserialize",
    Message::new_request(bb!(Request::Bye), bb!(55))
);

criterion_group!(
    benches,
    benchmark_request_ping_deserialization,
    benchmark_request_ping_deserialization,
    benchmark_request_handshake_deserialization,
    benchmark_request_post_results_deserialization,
    benchmark_request_post_stats_deserialization,
    benchmark_request_send_notification_deserialization,
    benchmark_request_get_settings_deserialization,
    benchmark_request_update_check_deserialization,
    benchmark_request_send_notification_deserialization,
    benchmark_request_update_check_deserialization,
    benchmark_request_next_update_chunk_deserialization,
    benchmark_request_report_fw_update_deserialization,
    benchmark_request_bye_deserialization
);
criterion_main!(benches);
