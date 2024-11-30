use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
  sync::Arc,
  time::Instant,
};

use common::create_di::create_di;
use ioc_container_rs::{
  errors::error::Error,
  ports::{adapter_port::AdapterPort, context_port::ContextPort},
};
use tests::adapters::adapter_nested_test::{AdapterNested, AdapterNestedPort};
use tokio::task::JoinHandle;

use crate::common::constants::{ITERATIONS, THREADS};

mod common;

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

async fn write_data(context: &Arc<dyn ContextPort>) -> Result<(), Error> {
  let svc = AdapterNested::get_adapter(context).await?;

  svc.set_string(&generate_random_string(10)).await?;

  Ok(())
}

async fn bench() -> Vec<JoinHandle<()>> {
  let mut handlers: Vec<JoinHandle<()>> = vec![];

  for _ in 0..THREADS {
    let handle = tokio::spawn(async {
      let context = create_di().await;

      if context.is_err() {
        eprintln!("Error creating DI: {:?}", context.err().unwrap());
        return;
      }

      let r = write_data(&context.unwrap()).await;

      if r.is_err() {
        eprintln!("Error writing data: {:?}", r.err().unwrap());
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
      "Write the data from DI\nThreads per iteration: {}\nIterations: {}\nDuration: {:?}\n====================\n",
      THREADS, ITERATIONS, duration
    )
  );
}
