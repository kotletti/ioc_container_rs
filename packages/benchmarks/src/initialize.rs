mod common;

use std::time::Instant;

use common::{constants::THREADS, create_di::create_di};
use tokio::task::JoinHandle;

use crate::common::constants::ITERATIONS;

async fn bench() -> Vec<JoinHandle<()>> {
  let mut handlers: Vec<JoinHandle<()>> = vec![];

  for _ in 0..THREADS {
    let handle = tokio::spawn(async {
      create_di().await;
    });

    handlers.push(handle);
  }

  handlers
}

#[tokio::main]
async fn main() {
  let start = Instant::now();

  for _ in 0..ITERATIONS {
    let handlers = bench().await;

    for handle in handlers {
      handle.await.unwrap();
    }
  }

  let duration = start.elapsed();

  println!(
    "{}",
    format!(
      "Initialization of DI\nThreads per iteration: {}\nIterations: {}\nDuration: {:?}\n====================\n",
      THREADS, ITERATIONS, duration
    )
  );
}
