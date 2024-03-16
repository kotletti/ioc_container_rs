use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::context::{container_context::ContainerContext, context::Context};

use crate::{
  entities::user_entity::UserEntity,
  ports::{
    input::{
      add_user_port::AddUserPort, delete_user_port::DeleteUserPort, get_user_port::GetUserPort,
    },
    output::{user_repository_port::UserRepositoryPort, user_service_port::UserServicePort},
  },
};

use super::user_repository_adapter;

pub struct Adapter {
  context: Arc<ContainerContext>,
}

impl Adapter {
  pub fn new(context: Arc<ContainerContext>) -> Self {
    Self { context }
  }

  pub fn token() -> &'static str {
    "USER_SERVICE_ADAPTER"
  }

  async fn get_user_repository(&self) -> user_repository_adapter::Adapter {
    self
      .context
      .resolve_provider::<user_repository_adapter::Adapter>(
        user_repository_adapter::Adapter::token(),
      )
      .await
  }
}

#[async_trait]
impl UserServicePort for Adapter {
  async fn add_user(&self, payload: &AddUserPort) -> Result<UserEntity, String> {
    let user_entity = UserEntity {
      name: payload.name.to_string(),
      email: payload.email.to_string(),
    };

    let user_repository = self.get_user_repository().await;

    match user_repository.add_user(&user_entity).await {
      Ok(_) => Ok(user_entity),
      Err(err) => Err(err),
    }
  }

  async fn delete_user(&self, payload: &DeleteUserPort) -> Result<(), String> {
    let user_repository = self.get_user_repository().await;

    let user_entity = match user_repository.get_user_by_email(&payload.email).await {
      Ok(Some(user)) => user,
      Ok(None) => return Err("User not found".to_string()),
      Err(err) => return Err(err),
    };

    user_repository.delete_user(&user_entity).await
  }

  async fn get_user(&self, payload: &GetUserPort) -> Result<Option<UserEntity>, String> {
    let user_repository = self.get_user_repository().await;

    user_repository.get_user_by_email(&payload.email).await
  }

  async fn get_count(&self) -> Result<usize, String> {
    let user_repository = self.get_user_repository().await;

    user_repository.get_count().await
  }
}
