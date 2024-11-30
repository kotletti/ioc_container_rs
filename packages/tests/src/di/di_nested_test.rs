#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use ioc_container_rs::container::di::{InjectAdapter, DI};
  use ioc_container_rs::context::container_context::ContainerContext;
  use ioc_container_rs::errors::error::Error;
  use ioc_container_rs::ports::adapter_port::AdapterPort;

  use crate::adapters::adapter_nested_test::{AdapterNested, AdapterNestedPort};
  use crate::adapters::adapter_number_test::AdapterNumberTest;
  use crate::adapters::adapter_string_test::AdapterStringTest;

  const DEFAULT_STRING: &str = "Hello, World!";
  const DEFAULT_NUMBER: i32 = 0;

  async fn create_di() -> Result<DI, Error> {
    DI::new(Arc::new(ContainerContext::new()))
      .inject(InjectAdapter {
        token: AdapterStringTest::token(),
        factory: Arc::new(|_| AdapterStringTest::new()),
      })
      .await?
      .inject(InjectAdapter {
        token: AdapterNumberTest::token(),
        factory: Arc::new(|_| AdapterNumberTest::new()),
      })
      .await?
      .inject(InjectAdapter {
        token: AdapterNested::token(),
        factory: Arc::new(|context| AdapterNested::new(context)),
      })
      .await
  }

  #[tokio::test]
  async fn should_be_build_and_resolve_service_nested() {
    let di = create_di().await;

    assert_eq!(di.is_ok(), true);

    let di = di.unwrap();

    let context = di.get_context();

    let svc = AdapterNested::get_adapter(&context).await;

    assert_eq!(svc.is_ok(), true);
  }

  #[tokio::test]
  async fn should_be_access_to_nested_services() {
    let di = create_di().await;

    assert_eq!(di.is_ok(), true);

    let di = di.unwrap();

    let context = di.get_context();

    let svc = AdapterNested::get_adapter(&context).await;

    assert_eq!(svc.is_ok(), true);

    let svc = svc.unwrap();

    let number = svc.get_number().await;
    let string = svc.get_string().await;

    assert_eq!(number.is_ok(), true);
    assert_eq!(string.is_ok(), true);

    let number = number.unwrap();
    let string = string.unwrap();

    assert_eq!(number, DEFAULT_NUMBER);
    assert_eq!(string, DEFAULT_STRING);
  }
}
