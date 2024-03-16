use std::sync::Arc;

use ioc_container_rs::{
  container::di::{InjectAdapter, DI},
  context::container_context::ContainerContext,
};
use tests::adapters::{
  adapter_nested_test::AdapterNested, adapter_number_test::AdapterNumberTest,
  adapter_string_test::AdapterStringTest,
};

pub async fn create_di() -> Arc<ContainerContext> {
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
    .await
    .inject(InjectAdapter {
      token: AdapterNested::token(),
      factory: Arc::new(AdapterNested::new),
    })
    .await;

  di.get_context()
}
