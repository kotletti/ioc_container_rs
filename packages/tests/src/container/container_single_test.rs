#[cfg(test)]
mod tests {
  use ioc_container_rs::{container::container::Container, ports::adapter_port::AdapterPort};

  use crate::adapters::adapter_string_test::AdapterStringTest;

  #[tokio::test]
  async fn should_be_able_to_register_and_resolve() {
    let container = Container::new();

    let registered = container
      .register(AdapterStringTest::token(), || AdapterStringTest::new())
      .await;

    assert_eq!(registered.is_ok(), true);

    let svc = container.resolve(AdapterStringTest::token()).await;

    assert_eq!(svc.is_ok(), true);
  }
}
