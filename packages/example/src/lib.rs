use std::sync::Arc;

use adapters::{
  user_repository_adapter::UserRepositoryAdapter, user_service_adapter::UserServiceAdapter,
};
use entities::user_entity::UserEntity;
use ioc_container_rs::{
  container::di::{InjectAdapter, DI},
  context::container_context::ContainerContext,
  errors::error::Error,
  ports::adapter_port::AdapterPort,
};
use tokio::sync::RwLock;

pub mod adapters;
pub mod entities;
pub mod ports;

pub async fn create_di() -> Result<DI, Error> {
  let di = DI::new(Arc::new(ContainerContext::new()));

  let store: Arc<RwLock<Vec<UserEntity>>> = Arc::new(RwLock::new(vec![]));

  let user_repository_injector = InjectAdapter {
    token: UserRepositoryAdapter::token(),
    factory: Arc::new(move |_| UserRepositoryAdapter::new(store.clone())),
  };

  let di = di
    .inject(user_repository_injector)
    .await?
    .inject(InjectAdapter {
      token: UserServiceAdapter::token(),
      factory: Arc::new(UserServiceAdapter::new),
    })
    .await;

  di
}

#[cfg(test)]
mod tests {

  use ioc_container_rs::ports::adapter_port::AdapterPort;

  use crate::{
    adapters::{
      user_repository_adapter::UserRepositoryAdapter, user_service_adapter::UserServiceAdapter,
    },
    create_di,
    ports::{
      input::{
        add_user_port::AddUserPort, delete_user_port::DeleteUserPort, get_user_port::GetUserPort,
      },
      output::{user_repository_port::UserRepositoryPort, user_service_port::UserServicePort},
    },
  };

  #[tokio::test]
  async fn should_be_return_zero_count_from_repository() {
    let di = create_di().await;

    assert_eq!(di.is_ok(), true);

    let di = di.unwrap();

    let context = di.get_context();

    let user_repository = UserRepositoryAdapter::get_adapter(&context).await;

    assert_eq!(user_repository.is_ok(), true);

    let user_repository = user_repository.unwrap();

    let count = user_repository.get_count().await;

    assert_eq!(count.is_ok(), true);

    let count = count.unwrap();

    assert_eq!(count, 0);
  }

  #[tokio::test]
  async fn should_be_return_zero_count_from_service() {
    let di = create_di().await;

    assert_eq!(di.is_ok(), true);

    let di = di.unwrap();

    let context = di.get_context();

    let user_service = UserServiceAdapter::get_adapter(&context).await;

    assert_eq!(user_service.is_ok(), true);

    let user_service = user_service.unwrap();

    let count = user_service.get_count().await;

    assert_eq!(count.is_ok(), true);

    let count = count.unwrap();

    assert_eq!(count, 0);
  }

  #[tokio::test]
  async fn should_return_new_user() {
    let di = create_di().await;

    assert_eq!(di.is_ok(), true);

    let di = di.unwrap();

    let context = di.get_context();

    let user_service = UserServiceAdapter::get_adapter(&context).await;

    assert_eq!(user_service.is_ok(), true);

    let user_service = user_service.unwrap();

    let user_entity = user_service
      .add_user(&AddUserPort {
        name: "Andrey".to_string(),
        email: "andrey@mail.ru".to_string(),
      })
      .await;

    assert_eq!(user_entity.is_ok(), true);

    let user_entity = user_entity.unwrap();

    let count = user_service.get_count().await;

    assert_eq!(count.is_ok(), true);

    let count = count.unwrap();

    assert_eq!(count, 1);
    assert_eq!(user_entity.name, "Andrey");
  }

  #[tokio::test]
  async fn should_remove_new_user() {
    let di = create_di().await;

    assert_eq!(di.is_ok(), true);

    let di = di.unwrap();

    let context = di.get_context();

    let user_service = UserServiceAdapter::get_adapter(&context).await;

    assert_eq!(user_service.is_ok(), true);

    let user_service = user_service.unwrap();

    let user_entity = user_service
      .add_user(&AddUserPort {
        name: "Andrey".to_string(),
        email: "andrey@mail.ru".to_string(),
      })
      .await;

    assert_eq!(user_entity.is_ok(), true);

    let user_entity = user_entity.unwrap();

    let user_deletion = user_service
      .delete_user(&DeleteUserPort {
        email: user_entity.email.to_string(),
      })
      .await;

    assert_eq!(user_deletion.is_ok(), true);

    let check_user_entity = user_service
      .get_user(&GetUserPort {
        email: user_entity.email.to_string(),
      })
      .await;

    assert_eq!(check_user_entity.is_ok(), true);

    let check_user_entity = check_user_entity.unwrap();

    assert_eq!(check_user_entity.is_none(), true);
  }
}
