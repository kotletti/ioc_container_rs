#[cfg(test)]
mod tests {
  use ioc_container_rs::container::container::Container;

  use crate::adapters::adapter_string_test::AdapterStringTest;

  #[tokio::test]
  async fn should_be_able_to_register_and_resolve() {
    let container = Container::new();

    container
      .register(AdapterStringTest::token(), || AdapterStringTest::new())
      .await;

    let svc = container
      .resolve::<AdapterStringTest>(AdapterStringTest::token())
      .await;

    assert_eq!(svc.is_some(), true);
  }
}
