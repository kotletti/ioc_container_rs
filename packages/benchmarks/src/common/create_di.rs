use std::sync::Arc;

use ioc_container_rs::{
  container::di::{InjectAdapter, DI},
  context::container_context::ContainerContext,
  errors::error::Error,
  ports::{adapter_port::AdapterPort, context_port::ContextPort},
};
use tests::adapters::{
  adapter_nested_test::AdapterNested, adapter_number_test::AdapterNumberTest,
  adapter_string_test::AdapterStringTest,
};

pub async fn create_di() -> Result<Arc<dyn ContextPort>, Error> {
  let di = DI::new(Arc::new(ContainerContext::new()));

  let di = di
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
      factory: Arc::new(AdapterNested::new),
    })
    .await?;

  Ok(di.get_context())
}
