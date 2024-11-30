use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::{
  context::{container_context::ContainerContext, context::Context},
  errors::error::Error,
};

use crate::{
  entities::user_entity::UserEntity,
  ports::{
    input::{
      add_user_port::AddUserPort, delete_user_port::DeleteUserPort, get_user_port::GetUserPort,
    },
    output::{user_repository_port::UserRepositoryPort, user_service_port::UserServicePort},
  },
};

use super::user_repository_adapter::UserRepositoryAdapter;

pub struct UserServiceAdapter {
  context: Arc<ContainerContext>,
}

impl UserServiceAdapter {
  pub fn new(context: Arc<ContainerContext>) -> Self {
    Self { context }
  }

  pub fn token() -> &'static str {
    "USER_SERVICE_ADAPTER"
  }

  async fn get_user_repository(&self) -> Result<Box<UserRepositoryAdapter>, Error> {
    self
      .context
      .resolve_provider::<UserRepositoryAdapter>(UserRepositoryAdapter::token())
      .await
  }
}

#[async_trait]
impl UserServicePort for UserServiceAdapter {
  async fn add_user(&self, payload: &AddUserPort) -> Result<UserEntity, Error> {
    let user_entity = UserEntity {
      name: payload.name.to_string(),
      email: payload.email.to_string(),
    };

    let user_repository = self.get_user_repository().await?;

    match user_repository.add_user(&user_entity).await {
      Ok(_) => Ok(user_entity),
      Err(e) => Err(e.into()),
    }
  }

  async fn delete_user(&self, payload: &DeleteUserPort) -> Result<(), Error> {
    let user_repository = self.get_user_repository().await?;

    let user_entity = match user_repository.get_user_by_email(&payload.email).await {
      Ok(Some(user)) => user,
      Ok(None) => return Err("User not found".into()),
      Err(e) => return Err(e.into()),
    };

    user_repository.delete_user(&user_entity).await
  }

  async fn get_user(&self, payload: &GetUserPort) -> Result<Option<UserEntity>, Error> {
    let user_repository = self.get_user_repository().await?;

    user_repository.get_user_by_email(&payload.email).await
  }

  async fn get_count(&self) -> Result<usize, Error> {
    let user_repository = self.get_user_repository().await?;

    user_repository.get_count().await
  }
}
