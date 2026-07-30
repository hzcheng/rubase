use std::hint::black_box;
use std::time::{Duration, Instant};

const ITERATIONS: u32 = 1_000_000;

fn main() {
    let started = Instant::now();
    for _ in 0..ITERATIONS {
        black_box(rubase::version());
    }

    report(started.elapsed());
}

fn report(elapsed: Duration) {
    println!("rubase version lookup: {ITERATIONS} iterations in {elapsed:?}");
}
