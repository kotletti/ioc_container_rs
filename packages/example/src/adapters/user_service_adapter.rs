use std::sync::Arc;

use async_trait::async_trait;
use example_domain::{
    entities::user_entity::UserEntity,
    ports::{user_repository_port::UserRepositoryPort, user_service_port::UserServicePort},
};
use ioc_container_rs::{
    adapters::{app_context::AppContext, di::DI},
    errors::error::Error,
    ports::adapter_port::AdapterPort,
};

pub struct UserServiceAdapter {
    user_repository: Arc<dyn UserRepositoryPort>,
}

impl UserServiceAdapter {
    pub fn new(user_repository: Arc<dyn UserRepositoryPort>) -> Self {
        Self { user_repository }
    }
}

impl AdapterPort<dyn UserServicePort> for UserServiceAdapter {
    fn construct(context: &Arc<AppContext>) -> Result<Arc<dyn UserServicePort>, Error> {
        let user_repository = context.resolve::<dyn UserRepositoryPort>()?;

        Ok(Arc::new(UserServiceAdapter::new(user_repository)))
    }

    async fn create_di(di: DI) -> Result<DI, Error> {
        di.bind::<dyn UserServicePort, Self>()
    }
}

#[async_trait]
impl UserServicePort for UserServiceAdapter {
    async fn add_user(&self, id: u32, name: &str, email: &str) -> Result<UserEntity, Error> {
        let user_entity = UserEntity {
            id,
            name: name.to_owned(),
            email: email.to_owned(),
        };

        self.user_repository.add_user(&user_entity).await?;

        Ok(user_entity)
    }

    async fn delete_user(&self, id: u32) -> Result<(), Error> {
        let user_entity = self.user_repository.get_user_by_id(id).await?;

        self.user_repository.delete_user(&user_entity).await
    }

    async fn get_user(&self, id: u32) -> Result<UserEntity, Error> {
        self.user_repository.get_user_by_id(id).await
    }

    async fn get_count(&self) -> Result<usize, Error> {
        self.user_repository.get_count().await
    }
}
