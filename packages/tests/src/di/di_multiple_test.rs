#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use ioc_container_rs::container::di::{InjectAdapter, DI};
  use ioc_container_rs::context::container_context::ContainerContext;
  use ioc_container_rs::ports::adapter_port::AdapterPort;

  use crate::adapters::adapter_number_test::AdapterNumberTest;
  use crate::adapters::adapter_string_test::AdapterStringTest;

  fn create_di() -> DI {
    DI::new(Arc::new(ContainerContext::new()))
  }

  #[tokio::test]
  async fn should_be_build_and_resolve_service_multiple() {
    let di = create_di();

    let di = di
      .inject(InjectAdapter {
        token: AdapterStringTest::token(),
        factory: Arc::new(|_| AdapterStringTest::new()),
      })
      .await;

    assert_eq!(di.is_ok(), true);

    let di = di
      .unwrap()
      .inject(InjectAdapter {
        token: AdapterNumberTest::token(),
        factory: Arc::new(|_| AdapterNumberTest::new()),
      })
      .await;

    assert_eq!(di.is_ok(), true);

    let di = di.unwrap();

    let context = di.get_context();

    let string_svc = AdapterStringTest::get_adapter(&context).await;

    assert_eq!(string_svc.is_ok(), true);

    let number_svc = AdapterNumberTest::get_adapter(&context).await;

    assert_eq!(number_svc.is_ok(), true)
  }
}
