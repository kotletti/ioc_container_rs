#[cfg(test)]
mod tests {
  use ioc_container_rs::{container::container::Container, ports::adapter_port::AdapterPort};

  use crate::adapters::{
    adapter_number_test::AdapterNumberTest, adapter_string_test::AdapterStringTest,
  };

  #[tokio::test]
  async fn should_be_able_to_register_and_resolve_multiple() {
    let container = Container::new();

    let registered = container
      .register(AdapterStringTest::token(), || AdapterStringTest::new())
      .await;

    assert_eq!(registered.is_ok(), true);

    let registered = container
      .register(AdapterNumberTest::token(), || AdapterNumberTest::new())
      .await;

    assert_eq!(registered.is_ok(), true);

    let first = container.resolve(AdapterStringTest::token()).await;

    let second = container.resolve(AdapterStringTest::token()).await;

    assert_eq!(first.is_ok(), true);
    assert_eq!(second.is_ok(), true);
  }
}
