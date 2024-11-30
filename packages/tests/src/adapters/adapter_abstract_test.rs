use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::{
  context::{container_context::ContainerContext, context::Context},
  errors::error::Error,
  ports::adapter_port::AdapterPort,
};

pub trait AdapterAbstractTestPort: Send + Sync + 'static {
  fn get_number(&self) -> i32;
  fn set_number(&mut self, number: i32);
}

pub struct AdapterAbstractTest {
  number: i32,
}

impl AdapterAbstractTest {
  pub fn new() -> Self {
    Self { number: 0 }
  }
}

#[async_trait]
impl AdapterPort<AdapterAbstractTest> for AdapterAbstractTest {
  fn token() -> &'static str {
    "ADAPTER_ABSTRACT_TEST"
  }

  async fn get_adapter(context: &Arc<ContainerContext>) -> Result<Box<Self>, Error> {
    let me = context.resolve_provider::<Self>(Self::token()).await?;

    Ok(me)
  }
}

impl AdapterAbstractTestPort for AdapterAbstractTest {
  fn get_number(&self) -> i32 {
    self.number
  }

  fn set_number(&mut self, number: i32) {
    self.number = number;
  }
}
