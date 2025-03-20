#![no_main]

use libfuzzer_sys::fuzz_target;
use pwmp_msg::version::Version;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = Version::parse(s);
    }
});
