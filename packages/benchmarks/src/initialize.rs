use std::hint::black_box;

use benchmarks::common::{
    bench::median,
    constants::{ITERATIONS, ROUNDS, THREADS},
    create_di::{create_di, create_graph},
};

#[tokio::main]
async fn main() {
    let di = create_di().await;

    let (without_di, without_di_failures) = median(&|| {
        black_box(create_graph());

        true
    });

    let (with_di, with_di_failures) = median(&|| match &di {
        Ok(ctx) => {
            black_box(&ctx);

            true
        }
        Err(_) => false,
    });

    println!(
        "Initialization of DI\n\
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
