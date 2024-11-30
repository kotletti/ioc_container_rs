use std::{sync::Arc, time::Instant};

use common::create_di::create_di;
use ioc_container_rs::{
  context::container_context::ContainerContext, errors::error::Error,
  ports::adapter_port::AdapterPort,
};
use tests::adapters::adapter_nested_test::{AdapterNested, AdapterNestedPort};
use tokio::task::JoinHandle;

use crate::common::constants::{ITERATIONS, THREADS};

mod common;

async fn read_data(context: &Arc<ContainerContext>) -> Result<(), Error> {
  let svc = AdapterNested::get_adapter(context).await?;

  svc.get_string().await?;

  Ok(())
}

async fn bench() -> Vec<JoinHandle<()>> {
  let mut handlers: Vec<JoinHandle<()>> = vec![];

  for _ in 0..THREADS {
    let handle = tokio::spawn(async {
      let context = create_di().await;

      if context.is_err() {
        eprintln!("Error: {:?}", &context.err());
        return;
      }

      let r = read_data(&context.unwrap()).await;

      if r.is_err() {
        eprintln!("Error: {:?}", &r.err());
      }
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
