use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::{
  errors::error::Error,
  ports::{adapter_port::AdapterPort, context_port::ContextPort},
};
use tokio::sync::RwLock;

use crate::{
  entities::user_entity::UserEntity, ports::output::user_repository_port::UserRepositoryPort,
};

pub struct UserRepositoryAdapter {
  store: Arc<RwLock<Vec<UserEntity>>>,
}

impl UserRepositoryAdapter {
  pub fn new(store: Arc<RwLock<Vec<UserEntity>>>) -> Self {
    Self { store }
  }
}

#[async_trait]
impl AdapterPort<UserRepositoryAdapter> for UserRepositoryAdapter {
  fn token() -> &'static str {
    "USER_REPOSITORY_ADAPTER"
  }

  async fn get_adapter(context: &Arc<dyn ContextPort>) -> Result<Box<Self>, Error> {
    let me = context
      .resolve_provider(Self::token())
      .await?
      .downcast::<Self>()
      .map_err(|_| format!("Cant resolve provider: {}", Self::token()))?;

    Ok(me)
  }
}

#[async_trait]
impl UserRepositoryPort for UserRepositoryAdapter {
  async fn add_user(&self, entity: &UserEntity) -> Result<(), Error> {
    let mut store = self.store.write().await;

    store.push(entity.clone());

    Ok(())
  }

  async fn delete_user(&self, entity: &UserEntity) -> Result<(), Error> {
    let mut store = self.store.write().await;

    let index = store.iter().position(|u| u.email == entity.email);

    match index {
      Some(i) => {
        store.remove(i);
        Ok(())
      }
      None => Err("User not found for delete.".into()),
    }
  }

  async fn get_user_by_email(&self, email: &str) -> Result<Option<UserEntity>, Error> {
    let store = self.store.read().await;

    let index = store.iter().position(|u| u.email == email);

    match index {
      Some(i) => match store.get(i) {
        Some(user) => Ok(Some(user.clone())),
        _ => Ok(None),
      },
      _ => Ok(None),
    }
  }

  async fn get_count(&self) -> Result<usize, Error> {
    let store = self.store.read().await;

    Ok(store.len())
  }
}
