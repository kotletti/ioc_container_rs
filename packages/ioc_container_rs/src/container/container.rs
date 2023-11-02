use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct Container {
  dependencies:
    Mutex<HashMap<String, Arc<dyn Fn() -> Box<dyn Any + Send + Sync + 'static> + Send + Sync>>>,
}

impl Container {
  pub fn new() -> Self {
    Self {
      dependencies: Mutex::new(HashMap::new()),
    }
  }

  pub fn register<T, F>(&self, key: &str, factory: F)
  where
    T: 'static + Send + Sync,
    F: Fn() -> T + Send + Sync + 'static,
  {
    let boxed_factory: Arc<dyn Fn() -> Box<dyn Any + Send + Sync + 'static> + Send + Sync> =
      Arc::new(move || Box::new(factory()) as Box<dyn Any + Send + Sync>);

    self
      .dependencies
      .lock()
      .unwrap()
      .insert(key.to_string(), boxed_factory);
  }

  pub fn resolve<T>(&self, key: &str) -> Option<T>
  where
    T: 'static,
  {
    self.dependencies.lock().unwrap().get(key).map(|factory| {
      let boxed_instance = factory();
      *boxed_instance.downcast::<T>().unwrap()
    })
  }
}
