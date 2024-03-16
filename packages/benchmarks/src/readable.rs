use std::{sync::Arc, time::Instant};

use common::create_di::create_di;
use ioc_container_rs::context::{container_context::ContainerContext, context::Context};
use tests::adapters::adapter_nested_test::AdapterNested;
use tokio::task::JoinHandle;

use crate::common::constants::{ITERATIONS, THREADS};

mod common;

async fn read_data(context: &Arc<ContainerContext>) {
  let svc = context
    .resolve_provider::<AdapterNested>(AdapterNested::token())
    .await;

  svc.get_string().await;
}

async fn bench() -> Vec<JoinHandle<()>> {
  let mut handlers: Vec<JoinHandle<()>> = vec![];

  for _ in 0..THREADS {
    let handle = tokio::spawn(async {
      let context = create_di().await;

      read_data(&context).await;
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
      "Read the data from DI\nThreads per iteration: {}\nIterations: {}\nDuration: {:?}\n====================\n",
      THREADS, ITERATIONS, duration
    )
  );
}
