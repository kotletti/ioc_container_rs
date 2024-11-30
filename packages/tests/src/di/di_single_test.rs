#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use ioc_container_rs::{
    container::di::{InjectAdapter, DI},
    context::context::Context,
    ports::adapter_port::AdapterPort,
  };

  use crate::adapters::adapter_string_test::{AdapterStringTest, AdapterStringTestPort};

  #[tokio::test]
  async fn should_be_build_and_resolve_service() {
    let di = DI::new()
      .inject(InjectAdapter {
        token: AdapterStringTest::token(),
        factory: Arc::new(|_| AdapterStringTest::new()),
      })
      .await;

    assert_eq!(di.is_ok(), true);

    let di = di.unwrap();

    let context = di.get_context();

    let svc = context
      .resolve_provider::<AdapterStringTest>(AdapterStringTest::token())
      .await;

    assert_eq!(svc.is_ok(), true);
  }

  #[tokio::test]
  async fn should_be_mutate_string() {
    let di = DI::new()
      .inject(InjectAdapter {
        token: AdapterStringTest::token(),
        factory: Arc::new(|_| AdapterStringTest::new()),
      })
      .await;

    assert_eq!(di.is_ok(), true);

    let di = di.unwrap();

    let context = di.get_context();

    const NEW_STRING: &str = "Hello, Rust!";

    let svc = context
      .resolve_provider::<AdapterStringTest>(AdapterStringTest::token())
      .await;

    assert_eq!(svc.is_ok(), true);

    let mut svc = svc.unwrap();

    svc.set_message(NEW_STRING.to_string());

    let message = svc.get_message();

    assert_eq!(message, NEW_STRING);
  }
}
