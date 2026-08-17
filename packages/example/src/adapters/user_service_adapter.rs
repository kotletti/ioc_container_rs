use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::{
    errors::error::Error,
    ports::{adapter_port::AdapterPort, container_port::ContainerPort},
};

use crate::{
    entities::user_entity::UserEntity,
    ports::{
        input::{
            add_user_port::AddUserPort, delete_user_port::DeleteUserPort,
            get_user_port::GetUserPort,
        },
        output::{user_repository_port::UserRepositoryPort, user_service_port::UserServicePort},
    },
};

pub struct UserServiceAdapter {
    user_repository: Arc<dyn UserRepositoryPort>,
}

impl UserServiceAdapter {
    pub fn new(user_repository: Arc<dyn UserRepositoryPort>) -> Self {
        Self { user_repository }
    }
}

impl AdapterPort for dyn UserServicePort {
    fn construct(container: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
        let user_repository = <dyn UserRepositoryPort>::get_adapter(container)?;

        Ok(Arc::new(UserServiceAdapter::new(user_repository)))
    }
}

#[async_trait]
impl UserServicePort for UserServiceAdapter {
    async fn add_user(&self, payload: &AddUserPort) -> Result<UserEntity, Error> {
        let user_entity = UserEntity {
            name: payload.name.to_string(),
            email: payload.email.to_string(),
        };

        self.user_repository.add_user(&user_entity).await?;

        Ok(user_entity)
    }

    async fn delete_user(&self, payload: &DeleteUserPort) -> Result<(), Error> {
        let user_entity = self
            .user_repository
            .get_user_by_email(&payload.email)
            .await?
            .ok_or("User not found")?;

        self.user_repository.delete_user(&user_entity).await
    }

    async fn get_user(&self, payload: &GetUserPort) -> Result<Option<UserEntity>, Error> {
        self.user_repository.get_user_by_email(&payload.email).await
    }

    async fn get_count(&self) -> Result<usize, Error> {
        self.user_repository.get_count().await
    }
}
