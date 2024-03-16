use std::{any::Any, collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

pub type AsyncAny = Box<dyn Any + Send + Sync>;

pub type FactoryItem = Arc<dyn Fn() -> AsyncAny + Send + Sync + 'static>;

pub struct ContainerStore {
  store: Arc<RwLock<HashMap<String, FactoryItem>>>,
}

impl ContainerStore {
  pub fn new() -> Self {
    Self {
      store: Arc::new(RwLock::new(HashMap::new())),
    }
  }

  pub async fn add<T, F>(&self, token: &'static str, factory: F)
  where
    T: Send + Sync + 'static,
    F: Fn() -> T + Send + Sync + 'static,
  {
    let factory_wrapped: FactoryItem = Arc::new(move || Box::new(factory()));

    self
      .store
      .write()
      .await
      .insert(token.to_string(), factory_wrapped);
  }

  pub async fn get<T>(&self, token: &'static str) -> Option<T>
  where
    T: 'static,
  {
    self.store.read().await.get(token).and_then(|factory| {
      let factory_instance = factory();

      match factory_instance.downcast::<T>() {
        Ok(instance) => Some(*instance),
        Err(_) => None,
      }
    })
  }

  pub async fn has(&self, token: &'static str) -> bool {
    self.store.read().await.contains_key(token)
  }
}
