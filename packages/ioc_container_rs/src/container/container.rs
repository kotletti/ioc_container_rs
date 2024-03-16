use std::sync::Arc;

use super::container_store::ContainerStore;

pub struct Container {
  store: Arc<ContainerStore>,
}

impl Container {
  pub fn new() -> Self {
    Self {
      store: Arc::new(ContainerStore::new()),
    }
  }

  pub async fn register<T, F>(&self, token: &'static str, factory: F)
  where
    T: Send + Sync + 'static,
    F: Fn() -> T + Send + Sync + 'static,
  {
    self.store.add(token, factory).await;
  }

  pub async fn resolve<T>(&self, token: &'static str) -> Option<T>
  where
    T: 'static,
  {
    self.store.get::<T>(token).await
  }

  pub async fn has_provider(&self, token: &'static str) -> bool {
    self.store.has(&token).await
  }
}
