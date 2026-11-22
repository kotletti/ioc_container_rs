use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    hint::black_box,
};

use benchmarks::{
    adapters::nested_adapter::NestedPort,
    common::{
        bench::median,
        constants::{ITERATIONS, ROUNDS, THREADS},
        create_di::{create_di, create_graph},
    },
};
use ioc_container_rs::ports::context_port::ContextResolverPort;

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

#[tokio::main]
async fn main() {
    let graph = create_graph();
    let ctx = create_di().await.expect("Failed to create DI");
    let message = generate_random_string(10);

    let (without_di, without_di_failures) =
        median(&|| black_box(graph.set_string(&message).is_ok()));

    let (with_di, with_di_failures) = median(&|| {
        let service = match ctx.resolve::<dyn NestedPort>() {
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
