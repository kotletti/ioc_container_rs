use std::sync::Arc;

use async_trait::async_trait;
use example_domain::{
    entities::user_entity::UserEntity, ports::user_repository_port::UserRepositoryPort,
};
use ioc_container_rs::{
    adapters::{app_context::AppContext, di::DI},
    errors::error::Error,
    ports::adapter_port::AdapterPort,
};
use tokio::sync::RwLock;

pub struct UserRepositoryAdapter {
    store: Arc<RwLock<Vec<UserEntity>>>,
}

impl UserRepositoryAdapter {
    pub fn new(store: Arc<RwLock<Vec<UserEntity>>>) -> Self {
        Self { store }
    }
}

impl AdapterPort<dyn UserRepositoryPort> for UserRepositoryAdapter {
    fn construct(_: &Arc<AppContext>) -> Result<Arc<dyn UserRepositoryPort>, Error> {
        Ok(Arc::new(Self::new(Arc::new(RwLock::new(vec![])))))
    }

    async fn create_di(di: DI) -> Result<DI, Error> {
        di.bind::<dyn UserRepositoryPort, Self>()
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

        let index = store
            .iter()
            .position(|u| u.email == entity.email)
            .ok_or("User not found for delete.")?;

        store.remove(index);

        Ok(())
    }

    async fn get_user_by_id(&self, id: u32) -> Result<UserEntity, Error> {
        let store = self.store.read().await;

        let user = store
            .iter()
            .find(|u| u.id == id)
            .cloned()
            .ok_or(format!("Not found user by id: {id}"))?;

        Ok(user)
    }

    async fn get_count(&self) -> Result<usize, Error> {
        let store = self.store.read().await;

        Ok(store.len())
    }
}
