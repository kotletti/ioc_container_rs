use std::{
    hint::black_box,
    thread,
    time::{Duration, Instant},
};

use ioc_container_rs::ports::adapter_port::AdapterPort;

use benchmarks::{
    adapters::nested_adapter::NestedPort,
    common::{
        constants::{ITERATIONS, ROUNDS, THREADS},
        create_di::{create_di, create_graph},
    },
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
    let graph = create_graph();
    let container = create_di().expect("Failed to create DI");

    let (without_di, without_di_failures) = median(&|| match graph.get_string() {
        Ok(message) => {
            black_box(message);

            true
        }
        Err(_) => false,
    });

    let (with_di, with_di_failures) = median(&|| {
        let service = match <dyn NestedPort>::get_adapter(&container) {
            Ok(service) => service,
            Err(_) => return false,
        };

        match service.get_string() {
            Ok(message) => {
                black_box(message);

                true
            }
            Err(_) => false,
        }
    });

    println!(
        "{}",
        format!(
            "Read the data from DI\nThreads: {}\nIterations per thread: {}\nRounds: {}\nFailures: {}\nDuration without DI: {:?}\nDuration with DI: {:?}\nOverhead: {:?}\n====================\n",
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
