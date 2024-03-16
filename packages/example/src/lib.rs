use std::sync::Arc;

use adapters::{user_repository_adapter, user_service_adapter};
use entities::user_entity::UserEntity;
use ioc_container_rs::{
  container::di::{InjectAdapter, DI},
  context::{container_context::ContainerContext, context::Context},
};
use tokio::sync::RwLock;

mod adapters;
mod entities;
mod ports;

pub async fn create_di() -> DI {
  let store: Arc<RwLock<Vec<UserEntity>>> = Arc::new(RwLock::new(vec![]));

  let user_repository_injector = InjectAdapter {
    token: user_repository_adapter::Adapter::token(),
    factory: Arc::new(move |_| user_repository_adapter::Adapter::new(store.clone())),
  };

  let di = DI::new()
    .inject(user_repository_injector)
    .await
    .inject(InjectAdapter {
      token: user_service_adapter::Adapter::token(),
      factory: Arc::new(user_service_adapter::Adapter::new),
    })
    .await;

  di
}

pub async fn get_user_service(context: &Arc<ContainerContext>) -> user_service_adapter::Adapter {
  context
    .resolve_provider::<user_service_adapter::Adapter>(user_service_adapter::Adapter::token())
    .await
}

pub async fn get_user_repository(
  context: &Arc<ContainerContext>,
) -> user_repository_adapter::Adapter {
  context
    .resolve_provider::<user_repository_adapter::Adapter>(user_repository_adapter::Adapter::token())
    .await
}

#[cfg(test)]
mod tests {

  use crate::{
    create_di, get_user_repository, get_user_service,
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

    let context = di.get_context();

    let user_repository = get_user_repository(&context).await;

    let count = user_repository.get_count().await.unwrap();

    assert_eq!(count, 0);
  }

  #[tokio::test]
  async fn should_be_return_zero_count_from_service() {
    let di = create_di().await;

    let context = di.get_context();

    let user_service = get_user_service(&context).await;

    let count = user_service.get_count().await.unwrap();

    assert_eq!(count, 0);
  }

  #[tokio::test]
  async fn should_return_new_user() {
    let di = create_di().await;

    let context = di.get_context();

    let user_service = get_user_service(&context).await;

    let user_entity = user_service
      .add_user(&AddUserPort {
        name: "Andrey".to_string(),
        email: "andrey@mail.ru".to_string(),
      })
      .await
      .unwrap();

    let count = user_service.get_count().await.unwrap();

    assert_eq!(count, 1);
    assert_eq!(user_entity.name, "Andrey");
  }

  #[tokio::test]
  async fn should_remove_new_user() {
    let di = create_di().await;

    let context = di.get_context();

    let user_service = get_user_service(&context).await;

    let user_entity = user_service
      .add_user(&AddUserPort {
        name: "Andrey".to_string(),
        email: "andrey@mail.ru".to_string(),
      })
      .await
      .unwrap();

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
      .await
      .unwrap();

    assert_eq!(check_user_entity.is_none(), true);
  }
}
