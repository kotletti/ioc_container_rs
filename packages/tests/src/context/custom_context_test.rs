#[cfg(test)]
pub mod tests {
  use std::sync::Arc;

  use ioc_container_rs::{
    container::di::{InjectAdapter, DI},
    errors::error::Error,
    ports::adapter_port::AdapterPort,
  };

  use crate::{
    adapters::adapter_number_test::AdapterNumberTest, context::custom_context::CustomContext,
  };

  async fn create_di() -> Result<DI, Error> {
    let di = DI::new(Arc::new(CustomContext::new()));

    let di = di
      .inject(InjectAdapter {
        token: AdapterNumberTest::token(),
        factory: Arc::new(|_| AdapterNumberTest::new()),
      })
      .await?;

    Ok(di)
  }

  #[tokio::test]
  async fn should_be_valid_di() {
    let di = create_di().await;

    assert_eq!(di.is_ok(), true);
  }

  #[tokio::test]
  async fn should_be_able_to_resolve_adapter() {
    let di = create_di().await.unwrap();

    let context = di.get_context();

    let adapter = AdapterNumberTest::get_adapter(&context).await;

    assert_eq!(adapter.is_ok(), true);
  }

  #[tokio::test]
  async fn should_be_cast_to_context() {
    let di = create_di().await.unwrap();

    let context = di.get_context();

    let context = context.as_any().downcast_ref::<CustomContext>();

    assert_eq!(context.is_some(), true);
  }

  #[tokio::test]
  async fn should_be_empty_metrics() {
    let di = create_di().await.unwrap();

    let context = di.get_context();
    let context = context.as_any().downcast_ref::<CustomContext>();

    assert_eq!(context.is_some(), true);

    let context = context.unwrap();

    let metrics = context.get_metrics().await;

    let metrics = metrics.read().await;

    assert_eq!(metrics.len(), 0);
  }

  #[tokio::test]
  async fn should_be_able_to_increment_metrics() {
    let di = create_di().await.unwrap();

    let context = di.get_context();

    let _ = AdapterNumberTest::get_adapter(&context).await;
    let _ = AdapterNumberTest::get_adapter(&context).await;

    let context = context.as_any().downcast_ref::<CustomContext>();

    assert_eq!(context.is_some(), true);

    let context = context.unwrap();

    let metrics = context.get_metrics().await;

    let metrics = metrics.read().await;

    let count = metrics.get(&AdapterNumberTest::token().to_string());

    assert_eq!(count.is_some(), true);

    let count = count.unwrap();

    assert_eq!(*count, 2);
  }
}
