#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use ioc_container_rs::container::di::{InjectAdapter, DI};
  use ioc_container_rs::context::context::Context;

  use crate::adapters::adapter_string_test::AdapterStringTest;

  #[tokio::test]
  async fn should_be_build_and_resolve_service() {
    let di = DI::new()
      .inject(InjectAdapter {
        token: AdapterStringTest::token(),
        factory: Arc::new(|_| AdapterStringTest::new()),
      })
      .await;

    let context = di.get_context();

    context
      .resolve_provider::<AdapterStringTest>(AdapterStringTest::token())
      .await;

    assert!(true);
  }

  #[tokio::test]
  async fn should_be_mutate_string() {
    let di = DI::new()
      .inject(InjectAdapter {
        token: AdapterStringTest::token(),
        factory: Arc::new(|_| AdapterStringTest::new()),
      })
      .await;

    let context = di.get_context();

    const NEW_STRING: &str = "Hello, Rust!";

    let mut svc = context
      .resolve_provider::<AdapterStringTest>(AdapterStringTest::token())
      .await;

    svc.set_message(NEW_STRING.to_string());

    let message = svc.get_message();

    assert_eq!(message, NEW_STRING);
  }
}
