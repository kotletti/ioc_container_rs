#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use ioc_container_rs::container::di::{InjectAdapter, DI};
  use ioc_container_rs::context::context::Context;

  use crate::adapters::adapter_number_test::AdapterNumberTest;
  use crate::adapters::adapter_string_test::AdapterStringTest;

  #[tokio::test]
  async fn should_be_build_and_resolve_service_multiple() {
    let di = DI::new()
      .inject(InjectAdapter {
        token: AdapterStringTest::token(),
        factory: Arc::new(|_| AdapterStringTest::new()),
      })
      .await
      .inject(InjectAdapter {
        token: AdapterNumberTest::token(),
        factory: Arc::new(|_| AdapterNumberTest::new()),
      })
      .await;

    let context = di.get_context();

    context
      .resolve_provider::<AdapterStringTest>(AdapterStringTest::token())
      .await;

    context
      .resolve_provider::<AdapterNumberTest>(AdapterNumberTest::token())
      .await;

    assert!(true)
  }
}
