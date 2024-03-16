#[cfg(test)]
mod tests {
  use ioc_container_rs::container::container_store::ContainerStore;

  use crate::adapters::{
    adapter_number_test::AdapterNumberTest, adapter_string_test::AdapterStringTest,
  };

  #[tokio::test]
  async fn should_be_able_to_register_and_resolve_multiple() {
    let store = ContainerStore::new();

    store
      .add(AdapterStringTest::token(), AdapterStringTest::new)
      .await;

    store
      .add(AdapterNumberTest::token(), AdapterNumberTest::new)
      .await;

    let first = store
      .get::<AdapterStringTest>(AdapterStringTest::token())
      .await;

    let second = store
      .get::<AdapterNumberTest>(AdapterNumberTest::token())
      .await;

    assert_eq!(first.is_some(), true);
    assert_eq!(second.is_some(), true);
  }

  #[tokio::test]
  async fn should_be_able_to_get_value_multiple() {
    let store = ContainerStore::new();

    store
      .add(AdapterStringTest::token(), AdapterStringTest::new)
      .await;

    store
      .add(AdapterNumberTest::token(), AdapterNumberTest::new)
      .await;

    let first = store
      .get::<AdapterStringTest>(AdapterStringTest::token())
      .await
      .unwrap();

    let second = store
      .get::<AdapterNumberTest>(AdapterNumberTest::token())
      .await
      .unwrap();

    let first_value = first.get_message();
    let second_value = second.get_number();

    assert_eq!(first_value, "Hello, World!");
    assert_eq!(second_value, 0);
  }

  #[tokio::test]
  async fn should_be_able_to_set_value_multiple() {
    let store = ContainerStore::new();

    store
      .add(AdapterStringTest::token(), AdapterStringTest::new)
      .await;

    store
      .add(AdapterNumberTest::token(), AdapterNumberTest::new)
      .await;

    let mut first = store
      .get::<AdapterStringTest>(AdapterStringTest::token())
      .await
      .unwrap();

    let mut second = store
      .get::<AdapterNumberTest>(AdapterNumberTest::token())
      .await
      .unwrap();

    const MESSAGE: &str = "Hello, Rust!";
    const NUMBER: i32 = 1;

    first.set_message(MESSAGE.to_string());
    second.set_number(NUMBER);

    let first_value = first.get_message();
    let second_value = second.get_number();

    assert_eq!(first_value, MESSAGE);
    assert_eq!(second_value, NUMBER);
  }
}
