pub mod adapters;
pub mod entities;
pub mod ports;

#[cfg(test)]
mod tests {

  use std::sync::Arc;

  use ioc_container_rs::{
    container::di::{InjectAdapter, DI},
    context::container_context::ContainerContext,
    ports::adapter_port::AdapterPort,
  };
  use tokio::sync::RwLock;

  use crate::{
    adapters::{
      user_repository_adapter::UserRepositoryAdapter, user_service_adapter::UserServiceAdapter,
    },
    entities::user_entity::UserEntity,
    ports::{
      input::{
        add_user_port::AddUserPort, delete_user_port::DeleteUserPort, get_user_port::GetUserPort,
      },
      output::{user_repository_port::UserRepositoryPort, user_service_port::UserServicePort},
    },
  };

  async fn create_di() -> DI {
    let di = DI::new(Arc::new(ContainerContext::new()));

    let store: Arc<RwLock<Vec<UserEntity>>> = Arc::new(RwLock::new(vec![]));

    let user_repository_injector = InjectAdapter {
      token: UserRepositoryAdapter::token(),
      factory: Arc::new(move |_| UserRepositoryAdapter::new(store.clone())),
    };

    let di = di
      .inject(user_repository_injector)
      .await
      .expect("UserRepositoryAdapter should be injected");

    let di = di
      .inject(InjectAdapter {
        token: UserServiceAdapter::token(),
        factory: Arc::new(UserServiceAdapter::new),
      })
      .await
      .expect("UserServiceAdapter should be injected");

    di
  }

  #[tokio::test]
  async fn should_be_return_zero_count_from_repository() {
    let di = create_di().await;

    let context = di.get_context();

    let user_repository = UserRepositoryAdapter::get_adapter(&context).await;

    assert!(user_repository.is_ok());

    let user_repository = user_repository.expect("UserRepository should exist");

    let count = user_repository
      .get_count()
      .await
      .expect("Get count has error");

    assert_eq!(count, 0, "Count should be 0");
  }

  #[tokio::test]
  async fn should_be_return_zero_count_from_service() {
    let di = create_di().await;

    let context = di.get_context();

    let user_service = UserServiceAdapter::get_adapter(&context)
      .await
      .expect("UserService should exist");

    let count = user_service.get_count().await.expect("Get count has error");

    assert_eq!(count, 0, "Count should be 0");
  }

  #[tokio::test]
  async fn should_return_new_user() {
    let di = create_di().await;

    let context = di.get_context();

    let user_service = UserServiceAdapter::get_adapter(&context)
      .await
      .expect("UserService should exist");

    let user_entity = user_service
      .add_user(&AddUserPort {
        name: "Andrey".to_string(),
        email: "andrey@mail.ru".to_string(),
      })
      .await
      .expect("Add user has error");

    let count = user_service.get_count().await.expect("Get count has error");

    assert_eq!(count, 1, "Count should be 1");
    assert_eq!(user_entity.name, "Andrey", "Name should be Andrey");
  }

  #[tokio::test]
  async fn should_remove_new_user() {
    let di = create_di().await;

    let context = di.get_context();

    let user_service = UserServiceAdapter::get_adapter(&context)
      .await
      .expect("UserService should exist");

    let user_entity = user_service
      .add_user(&AddUserPort {
        name: "Andrey".to_string(),
        email: "andrey@mail.ru".to_string(),
      })
      .await
      .expect("Add user has error");

    user_service
      .delete_user(&DeleteUserPort {
        email: user_entity.email.to_string(),
      })
      .await
      .expect("Delete user has error");

    let check_user_entity = user_service
      .get_user(&GetUserPort {
        email: user_entity.email.to_string(),
      })
      .await
      .expect("Get user has error");

    assert!(
      check_user_entity.is_none(),
      "User after deletion should be None"
    );
  }
}
