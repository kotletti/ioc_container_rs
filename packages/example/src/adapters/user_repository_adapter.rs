use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
  entities::user_entity::UserEntity, ports::output::user_repository_port::UserRepositoryPort,
};

pub struct Adapter {
  store: Arc<RwLock<Vec<UserEntity>>>,
}

impl Adapter {
  pub fn new(store: Arc<RwLock<Vec<UserEntity>>>) -> Self {
    Self { store }
  }

  pub fn token() -> &'static str {
    "USER_REPOSITORY_ADAPTER"
  }
}

#[async_trait]
impl UserRepositoryPort for Adapter {
  async fn add_user(&self, entity: &UserEntity) -> Result<(), String> {
    let mut store = self.store.write().await;

    store.push(entity.clone());

    Ok(())
  }

  async fn delete_user(&self, entity: &UserEntity) -> Result<(), String> {
    let mut store = self.store.write().await;

    let index = store.iter().position(|u| u.email == entity.email);

    match index {
      Some(i) => {
        store.remove(i);
        Ok(())
      }
      None => Err("User not found for delete.".to_string()),
    }
  }

  async fn get_user_by_email(&self, email: &str) -> Result<Option<UserEntity>, String> {
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

  async fn get_user(&self, entity: &UserEntity) -> Result<Option<UserEntity>, String> {
    let store = self.store.read().await;

    let index = store.iter().position(|u| u.email == entity.email);

    match index {
      Some(i) => match store.get(i) {
        Some(user) => Ok(Some(user.clone())),
        _ => Ok(None),
      },
      _ => Ok(None),
    }
  }

  async fn get_count(&self) -> Result<usize, String> {
    let store = self.store.read().await;

    Ok(store.len())
  }
}
