#[cfg(test)]
mod tests {

  use ioc_container_rs::container::container_store::ContainerStore;

  use crate::adapters::adapter_abstract_test::AdapterAbstractTest;

  #[tokio::test]
  async fn should_be_able_to_register_and_resolve_abstract() {
    let store = ContainerStore::new();

    store
      .add(AdapterAbstractTest::token(), AdapterAbstractTest::new)
      .await;

    let svc = store
      .get::<AdapterAbstractTest>(AdapterAbstractTest::token())
      .await;

    assert_eq!(svc.is_some(), true);
  }
}
