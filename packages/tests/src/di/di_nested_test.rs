#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use ioc_container_rs::container::di::{InjectAdapter, DI};
  use ioc_container_rs::context::context::Context;

  use crate::adapters::adapter_nested_test::AdapterNested;
  use crate::adapters::adapter_number_test::AdapterNumberTest;
  use crate::adapters::adapter_string_test::AdapterStringTest;

  const DEFAULT_STRING: &str = "Hello, World!";
  const DEFAULT_NUMBER: i32 = 0;

  async fn create_di() -> DI {
    DI::new()
      .inject(InjectAdapter {
        token: AdapterStringTest::token(),
        factory: Arc::new(|_| AdapterStringTest::new()),
      })
      .await
      .inject(InjectAdapter {
        token: AdapterNumberTest::token(),
        factory: Arc::new(|_| AdapterNumberTest::new()),
      })
      .await
      .inject(InjectAdapter {
        token: AdapterNested::token(),
        factory: Arc::new(AdapterNested::new),
      })
      .await
  }

  #[tokio::test]
  async fn should_be_build_and_resolve_service_nested() {
    let di = create_di().await;

    let context = di.get_context();

    context
      .resolve_provider::<AdapterNested>(AdapterNested::token())
      .await;

    assert!(true);
  }

  #[tokio::test]
  async fn should_be_access_to_nested_services() {
    let di = create_di().await;

    let context = di.get_context();

    let svc = context
      .resolve_provider::<AdapterNested>(AdapterNested::token())
      .await;

    let number = svc.get_number().await;
    let string = svc.get_string().await;

    assert_eq!(number, DEFAULT_NUMBER);
    assert_eq!(string, DEFAULT_STRING);
  }
}
