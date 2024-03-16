#[cfg(test)]
mod tests {
  use ioc_container_rs::container::container::Container;

  use crate::adapters::{
    adapter_number_test::AdapterNumberTest, adapter_string_test::AdapterStringTest,
  };

  #[tokio::test]
  async fn should_be_able_to_register_and_resolve_multiple() {
    let container = Container::new();

    container
      .register(AdapterStringTest::token(), || AdapterStringTest::new())
      .await;

    container
      .register(AdapterNumberTest::token(), || AdapterNumberTest::new())
      .await;

    let first = container
      .resolve::<AdapterStringTest>(AdapterStringTest::token())
      .await;

    let second = container
      .resolve::<AdapterStringTest>(AdapterStringTest::token())
      .await;

    assert_eq!(first.is_some(), true);
    assert_eq!(second.is_some(), true);
  }
}
