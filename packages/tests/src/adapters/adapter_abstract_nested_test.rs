use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::context::{container_context::ContainerContext, context::Context};

use super::{adapter_number_test::AdapterNumberTest, adapter_string_test::AdapterStringTest};

#[async_trait]
pub trait AdapterAbstractNestedTestPort: Send + Sync {
  async fn set_number(&self, number: i32);
  async fn set_string(&self, message: &str);
  async fn get_string(&self) -> String;
  async fn get_number(&self) -> i32;
}

pub struct AdapterAbstractNestedTest {
  context: Arc<ContainerContext>,
}

impl AdapterAbstractNestedTest {
  pub fn new(context: Arc<ContainerContext>) -> Self {
    Self { context }
  }

  pub fn token() -> &'static str {
    "ADAPTER_ABSTRACT_NESTED_TEST"
  }
}

#[async_trait]
impl AdapterAbstractNestedTestPort for AdapterAbstractNestedTest {
  async fn set_number(&self, number: i32) {
    self
      .context
      .resolve_provider::<AdapterNumberTest>(AdapterNumberTest::token())
      .await
      .set_number(number);
  }

  async fn set_string(&self, message: &str) {
    self
      .context
      .resolve_provider::<AdapterStringTest>(AdapterStringTest::token())
      .await
      .set_message(message.to_string());
  }

  async fn get_string(&self) -> String {
    self
      .context
      .resolve_provider::<AdapterStringTest>(AdapterStringTest::token())
      .await
      .get_message()
      .to_string()
  }

  async fn get_number(&self) -> i32 {
    self
      .context
      .resolve_provider::<AdapterNumberTest>(AdapterNumberTest::token())
      .await
      .get_number()
  }
}
