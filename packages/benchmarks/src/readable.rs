use std::hint::black_box;

use benchmarks::{
    adapters::nested_adapter::NestedPort,
    common::{
        bench::median,
        constants::{ITERATIONS, ROUNDS, THREADS},
        create_di::{create_di, create_graph},
    },
};
use ioc_container_rs::ports::context_port::ContextResolverPort;

#[tokio::main]
async fn main() {
    let graph = create_graph();
    let ctx = create_di().await.expect("Failed to create DI");

    let (without_di, without_di_failures) = median(&|| match graph.get_string() {
        Ok(message) => {
            black_box(message);

            true
        }
        Err(_) => false,
    });

    let (with_di, with_di_failures) = median(&|| {
        let service = match ctx.resolve::<dyn NestedPort>() {
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
        "Read the data from DI\n\
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
