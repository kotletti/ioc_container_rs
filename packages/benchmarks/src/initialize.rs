use std::{
    hint::black_box,
    thread,
    time::{Duration, Instant},
};

use benchmarks::common::{
    constants::{ITERATIONS, ROUNDS, THREADS},
    create_di::{create_di, create_graph},
};

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

fn median(operation: &(impl Fn() -> bool + Sync)) -> (Duration, usize) {
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

fn main() {
    let (without_di, without_di_failures) = median(&|| {
        black_box(create_graph());

        true
    });

    let (with_di, with_di_failures) = median(&|| match create_di() {
        Ok(container) => {
            black_box(&container);

            true
        }
        Err(_) => false,
    });

    println!(
        "{}",
        format!(
            "Initialization of DI\nThreads: {}\nIterations per thread: {}\nRounds: {}\nFailures: {}\nDuration without DI: {:?}\nDuration with DI: {:?}\nOverhead: {:?}\n====================\n",
            THREADS,
            ITERATIONS,
            ROUNDS,
            without_di_failures + with_di_failures,
            without_di,
            with_di,
            with_di.saturating_sub(without_di)
        )
    );
}
