use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::{
  context::{container_context::ContainerContext, context::Context},
  errors::error::Error,
  ports::adapter_port::AdapterPort,
};

use super::{
  adapter_number_test::{AdapterNumberTest, AdapterNumberTestPort},
  adapter_string_test::{AdapterStringTest, AdapterStringTestPort},
};

#[async_trait]
pub trait AdapterAbstractNestedTestPort: Send + Sync {
  async fn set_number(&self, number: i32) -> Result<(), Error>;
  async fn set_string(&self, message: &str) -> Result<(), Error>;
  async fn get_string(&self) -> Result<String, Error>;
  async fn get_number(&self) -> Result<i32, Error>;
}

pub struct AdapterAbstractNestedTest {
  context: Arc<ContainerContext>,
}

impl AdapterAbstractNestedTest {
  pub fn new(context: Arc<ContainerContext>) -> Self {
    Self { context }
  }
}

#[async_trait]
impl AdapterPort<AdapterAbstractNestedTest> for AdapterAbstractNestedTest {
  fn token() -> &'static str {
    "ADAPTER_ABSTRACT_NESTED_TEST"
  }

  async fn get_adapter(context: &Arc<ContainerContext>) -> Result<Box<Self>, Error> {
    let me = context.resolve_provider::<Self>(Self::token()).await?;

    Ok(me)
  }
}

#[async_trait]
impl AdapterAbstractNestedTestPort for AdapterAbstractNestedTest {
  async fn set_number(&self, number: i32) -> Result<(), Error> {
    let mut svc = AdapterNumberTest::get_adapter(&self.context).await?;

    svc.set_number(number);

    Ok(())
  }

  async fn set_string(&self, message: &str) -> Result<(), Error> {
    let mut svc = AdapterStringTest::get_adapter(&self.context).await?;

    svc.set_message(message.to_string());

    Ok(())
  }

  async fn get_string(&self) -> Result<String, Error> {
    let provider = self
      .context
      .resolve_provider::<AdapterStringTest>(AdapterStringTest::token())
      .await?;

    let message = provider.get_message();

    Ok(message.to_string())
  }

  async fn get_number(&self) -> Result<i32, Error> {
    let provider = self
      .context
      .resolve_provider::<AdapterNumberTest>(AdapterNumberTest::token())
      .await?;

    let number = provider.get_number();

    Ok(number)
  }
}
