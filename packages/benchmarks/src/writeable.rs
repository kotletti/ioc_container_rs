use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
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

fn generate_random_string(len: usize) -> String {
    let mut result = String::with_capacity(len);
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut counter = 0;

    while result.len() < len {
        let mut hasher = DefaultHasher::new();
        counter.hash(&mut hasher);
        let hash = hasher.finish();

        let idx = (hash as usize) % charset.len();
        let char = charset.chars().nth(idx).expect("Should have a character");
        result.push(char);

        counter += 1;
    }

    result
}

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
    let message = generate_random_string(10);

    let (without_di, without_di_failures) =
        median(&|| black_box(graph.set_string(&message).is_ok()));

    let (with_di, with_di_failures) = median(&|| {
        let service = match <dyn NestedPort>::get_adapter(&container) {
            Ok(service) => service,
            Err(_) => return false,
        };

        black_box(service.set_string(&message).is_ok())
    });

    println!(
        "Write the data to DI\n\
         Threads: {}\n\
         Iterations per thread: {}\n\
         Rounds: {}\n\
         Failures: {}\n\
         Duration without DI: {:?}\n\
         Duration with DI: {:?}\n\
         Overhead: {:?}\n\
         ====================\n",
        THREADS,
        ITERATIONS,
        ROUNDS,
        without_di_failures + with_di_failures,
        without_di,
        with_di,
        with_di.saturating_sub(without_di)
    );
}
