#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use ioc_container_rs::container::di::{InjectAdapter, DI};
  use ioc_container_rs::context::context::Context;
  use ioc_container_rs::ports::adapter_port::AdapterPort;

  use crate::adapters::adapter_number_test::AdapterNumberTest;
  use crate::adapters::adapter_string_test::AdapterStringTest;

  #[tokio::test]
  async fn should_be_build_and_resolve_service_multiple() {
    let di = DI::new()
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

    let string_svc = context
      .resolve_provider::<AdapterStringTest>(AdapterStringTest::token())
      .await;

    assert_eq!(string_svc.is_ok(), true);

    let number_svc = context
      .resolve_provider::<AdapterNumberTest>(AdapterNumberTest::token())
      .await;

    assert_eq!(number_svc.is_ok(), true)
  }
}
