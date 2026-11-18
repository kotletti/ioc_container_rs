use std::{
    thread,
    time::{Duration, Instant},
};

use crate::common::constants::{ITERATIONS, ROUNDS, THREADS};

fn bench(operation: &(impl Fn() -> bool + Sync)) -> (Duration, usize) {
    let start = Instant::now();

    let failures = thread::scope(|scope| {
        let mut handlers = Vec::with_capacity(THREADS);

        for _ in 0..THREADS {
            let handle = scope.spawn(|| {
                let mut failures = 0;

                for _ in 0..ITERATIONS {
                    if !operation() {
                        failures += 1;
                    }
                }

                failures
            });

            handlers.push(handle);
        }

        handlers
            .into_iter()
            .map(|handle| handle.join().unwrap_or(ITERATIONS))
            .sum::<usize>()
    });

    (start.elapsed(), failures)
}

pub fn median(operation: &(impl Fn() -> bool + Sync)) -> (Duration, usize) {
    bench(operation);

    let mut rounds: Vec<Duration> = vec![];
    let mut failures = 0;

    for _ in 0..ROUNDS {
        let (duration, round_failures) = bench(operation);

        rounds.push(duration);
        failures += round_failures;
    }

    rounds.sort();

    (rounds[ROUNDS / 2], failures)
}
