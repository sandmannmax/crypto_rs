use std::time::Instant;
use std::time::Duration;

pub fn benchmark<T>(f: T) -> Duration where T: FnOnce() {
    let start = Instant::now();
    f();
    start.elapsed()
}

