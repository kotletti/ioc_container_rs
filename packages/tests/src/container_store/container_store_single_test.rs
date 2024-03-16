#[cfg(test)]
mod tests {
  use ioc_container_rs::container::container_store::ContainerStore;

  use crate::adapters::{
    adapter_number_test::AdapterNumberTest, adapter_string_test::AdapterStringTest,
  };

  #[tokio::test]
  async fn should_be_able_to_register_and_resolve() {
    let store = ContainerStore::new();

    store
      .add(AdapterStringTest::token(), AdapterStringTest::new)
      .await;

    let service = store
      .get::<AdapterStringTest>(AdapterStringTest::token())
      .await;

    assert_eq!(service.is_some(), true);
  }

  #[tokio::test]
  async fn should_be_get_correct_message() {
    let store = ContainerStore::new();

    store
      .add(AdapterStringTest::token(), AdapterStringTest::new)
      .await;

    let service = store
      .get::<AdapterStringTest>(AdapterStringTest::token())
      .await
      .unwrap();

    assert_eq!(service.get_message(), "Hello, World!");
  }

  #[tokio::test]
  async fn should_be_able_to_set_message() {
    let store = ContainerStore::new();

    store
      .add(AdapterStringTest::token(), AdapterStringTest::new)
      .await;

    let mut service = store
      .get::<AdapterStringTest>(AdapterStringTest::token())
      .await
      .unwrap();

    service.set_message("Hello, Rust!".to_string());

    assert_eq!(service.get_message(), "Hello, Rust!");
  }

  #[tokio::test]
  async fn should_be_get_correct_number() {
    let store = ContainerStore::new();

    store
      .add(AdapterNumberTest::token(), AdapterNumberTest::new)
      .await;

    let service = store
      .get::<AdapterNumberTest>(AdapterNumberTest::token())
      .await
      .unwrap();

    assert_eq!(service.get_number(), 0);
  }

  #[tokio::test]
  async fn should_be_able_to_set_number() {
    let store = ContainerStore::new();

    store
      .add(AdapterNumberTest::token(), AdapterNumberTest::new)
      .await;

    let mut service = store
      .get::<AdapterNumberTest>(AdapterNumberTest::token())
      .await
      .unwrap();

    service.set_number(1);

    assert_eq!(service.get_number(), 1);
  }
}
