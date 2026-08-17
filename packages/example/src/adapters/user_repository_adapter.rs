use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::{errors::error::Error, ports::adapter_port::AdapterPort};
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

impl AdapterPort for dyn UserRepositoryPort {}

#[async_trait]
impl UserRepositoryPort for UserRepositoryAdapter {
    async fn add_user(&self, entity: &UserEntity) -> Result<(), Error> {
        let mut store = self.store.write().await;

        store.push(entity.clone());

        Ok(())
    }

    async fn delete_user(&self, entity: &UserEntity) -> Result<(), Error> {
        let mut store = self.store.write().await;

        let index = store
            .iter()
            .position(|u| u.email == entity.email)
            .ok_or("User not found for delete.")?;

        store.remove(index);

        Ok(())
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<UserEntity>, Error> {
        let store = self.store.read().await;

        let user = store.iter().find(|u| u.email == email).cloned();

        Ok(user)
    }

    async fn get_count(&self) -> Result<usize, Error> {
        let store = self.store.read().await;

        Ok(store.len())
    }
}
