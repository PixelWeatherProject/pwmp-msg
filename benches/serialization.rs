use criterion::{criterion_group, criterion_main, Criterion};
use pwmp_msg::{mac::Mac, request::Request, version::Version, Message};
use std::hint::black_box;

fn benchmark_request_ping_serialization(c: &mut Criterion) {
    c.bench_function("Message(Request::Ping)::serialize", |b| {
        b.iter(|| Message::serialize(black_box(Message::new_request(Request::Ping, 88))))
    });
}

fn benchmark_request_handshake_serialization(c: &mut Criterion) {
    c.bench_function("Message(Request::Handshake)::serialize", |b| {
        b.iter(|| {
            Message::serialize(black_box(Message::new_request(
                Request::Handshake {
                    mac: Mac::new(0, 1, 2, 3, 4, 5),
                },
                47,
            )))
        })
    });
}

fn benchmark_request_post_results_serialization(c: &mut Criterion) {
    c.bench_function("Message(Request::PostResults)::serialize", |b| {
        b.iter(|| {
            Message::serialize(black_box(Message::new_request(
                Request::PostResults {
                    temperature: 45.78,
                    humidity: 45,
                    air_pressure: Some(65520),
                },
                88,
            )))
        })
    });
}

fn benchmark_request_post_stats_serialization(c: &mut Criterion) {
    c.bench_function("Message(Request::PostStats)::serialize", |b| {
        b.iter(|| {
            Message::serialize(black_box(Message::new_request(
                Request::PostStats {
                    battery: 4.20,
                    wifi_ssid: "Hello, World!".to_string().into_boxed_str(),
                    wifi_rssi: -45,
                },
                64,
            )))
        })
    });
}

fn benchmark_request_send_notification_serialization(c: &mut Criterion) {
    c.bench_function("Message(Request::SendNotification)::serialize", |b| {
        b.iter(|| {
            Message::serialize(black_box(Message::new_request(
                Request::SendNotification("Hello, World!".to_string().into_boxed_str()),
                88,
            )))
        })
    });
}

fn benchmark_request_get_settings_serialization(c: &mut Criterion) {
    c.bench_function("Message(Request::GetSettings)::serialize", |b| {
        b.iter(|| Message::serialize(black_box(Message::new_request(Request::GetSettings, 88))))
    });
}

fn benchmark_request_update_check_serialization(c: &mut Criterion) {
    c.bench_function("Message(Request::UpdateCheck)::serialize", |b| {
        b.iter(|| {
            Message::serialize(black_box(Message::new_request(
                Request::UpdateCheck(Version::new(5, 4, 7)),
                88,
            )))
        })
    });
}

fn benchmark_request_next_update_chunk_serialization(c: &mut Criterion) {
    c.bench_function("Message(Request::NextUpdateChunk)::serialize", |b| {
        b.iter(|| {
            Message::serialize(black_box(Message::new_request(
                Request::NextUpdateChunk(128_000_000),
                88,
            )))
        })
    });
}

fn benchmark_request_report_fw_update_serialization(c: &mut Criterion) {
    c.bench_function("Message(Request::ReportFirmwareUpdate)::serialize", |b| {
        b.iter(|| {
            Message::serialize(black_box(Message::new_request(
                Request::ReportFirmwareUpdate(true),
                88,
            )))
        })
    });
}

fn benchmark_request_bye_serialization(c: &mut Criterion) {
    c.bench_function("Message(Request::Bye)::serialize", |b| {
        b.iter(|| Message::serialize(black_box(Message::new_request(Request::Bye, 88))))
    });
}

criterion_group!(
    benches,
    benchmark_request_ping_serialization,
    benchmark_request_ping_serialization,
    benchmark_request_handshake_serialization,
    benchmark_request_post_results_serialization,
    benchmark_request_post_stats_serialization,
    benchmark_request_send_notification_serialization,
    benchmark_request_get_settings_serialization,
    benchmark_request_update_check_serialization,
    benchmark_request_send_notification_serialization,
    benchmark_request_update_check_serialization,
    benchmark_request_next_update_chunk_serialization,
    benchmark_request_report_fw_update_serialization,
    benchmark_request_bye_serialization
);
criterion_main!(benches);
