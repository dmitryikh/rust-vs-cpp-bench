use std::time::Instant;

// measure and return time elapsed in `func` in seconds
pub fn measure<F: FnOnce()>(func: F) -> f64 {
    let start = Instant::now();
    func();
    let elapsed = start.elapsed();
    (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0)
}

pub fn measure_and_print<F: FnOnce()>(func: F) {
    let m = measure(func);
    eprintln!("{}", m);
}
