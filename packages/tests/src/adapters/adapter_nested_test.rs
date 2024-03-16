use std::sync::Arc;

use ioc_container_rs::context::{container_context::ContainerContext, context::Context};

use super::{adapter_number_test::AdapterNumberTest, adapter_string_test::AdapterStringTest};

pub struct AdapterNested {
  context: Arc<ContainerContext>,
}

impl AdapterNested {
  pub fn new(context: Arc<ContainerContext>) -> Self {
    Self { context }
  }

  pub fn token() -> &'static str {
    "ADAPTER_NESTED_TEST"
  }

  pub async fn set_number(&self, number: i32) {
    self
      .context
      .resolve_provider::<AdapterNumberTest>(AdapterNumberTest::token())
      .await
      .set_number(number);
  }

  pub async fn set_string(&self, message: &str) {
    self
      .context
      .resolve_provider::<AdapterStringTest>(AdapterStringTest::token())
      .await
      .set_message(message.to_string());
  }

  pub async fn get_string(&self) -> String {
    self
      .context
      .resolve_provider::<AdapterStringTest>(AdapterStringTest::token())
      .await
      .get_message()
      .to_string()
  }

  pub async fn get_number(&self) -> i32 {
    self
      .context
      .resolve_provider::<AdapterNumberTest>(AdapterNumberTest::token())
      .await
      .get_number()
  }
}
