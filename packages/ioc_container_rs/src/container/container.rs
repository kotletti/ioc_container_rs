use std::{any::Any, collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::errors::error::Error;

pub type AsyncAny = Box<dyn Any + Send + Sync>;

pub type FactoryItem = Box<dyn Fn() -> AsyncAny + Send + Sync + 'static>;

type Store = Arc<RwLock<HashMap<String, FactoryItem>>>;

pub struct Container {
  store: Store,
}

impl Container {
  pub fn new() -> Self {
    let store = Arc::new(RwLock::new(HashMap::new()));

    Self { store }
  }

  pub async fn register<T, F>(&self, token: &'static str, factory: F) -> Result<(), Error>
  where
    T: Send + Sync + 'static,
    F: Fn() -> T + Send + Sync + 'static,
  {
    let has_token = self.has_token(token).await;

    if has_token {
      return Err(format!("Token {} already exists", token).into());
    }

    let mut store = self.store.write().await;

    let factory_wrapped: FactoryItem = Box::new(move || Box::new(factory()));

    store.insert(token.to_string(), factory_wrapped);

    Ok(())
  }

  pub async fn resolve<T: 'static>(&self, token: &'static str) -> Result<Box<T>, Error> {
    let has_token = self.has_token(token).await;

    if !has_token {
      return Err(format!("Token {} does not exist", token).into());
    }

    let store = self.store.read().await;

    let provider = store
      .get(token)
      .ok_or_else(|| format!("Provider by token {} does not exist", token))?;

    let provider = provider()
      .downcast::<T>()
      .map_err(|_| format!("Failed to cast token {} to expected type", token))?;

    Ok(provider)
  }

  pub async fn registered_tokens(&self) -> Vec<String> {
    self.store.read().await.keys().cloned().collect()
  }

  pub async fn has_token(&self, token: &'static str) -> bool {
    self.store.read().await.contains_key(token)
  }
}
