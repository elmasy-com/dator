#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // let now = std::time::Instant::now();
        // if dator::domain(s) {
        //     println!("Valid domain: {}  took={}ns", s, now.elapsed().as_nanos());
        // }
        dator::domain(s);
    }
});
