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
      .await
      .expect("Failed to inject AdapterStringTest");

    let di = di
      .inject(InjectAdapter {
        token: AdapterNumberTest::token(),
        factory: Arc::new(|_| AdapterNumberTest::new()),
      })
      .await
      .expect("Failed to inject AdapterNumberTest");

    let context = di.get_context();

    let string_svc = AdapterStringTest::get_adapter(&context).await;

    assert!(string_svc.is_ok());

    let number_svc = AdapterNumberTest::get_adapter(&context).await;

    assert!(number_svc.is_ok())
  }
}
