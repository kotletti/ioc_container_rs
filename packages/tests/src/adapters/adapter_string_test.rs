use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::{
  errors::error::Error,
  ports::{adapter_port::AdapterPort, context_port::ContextPort},
};

pub trait AdapterStringTestPort {
  fn get_message(&self) -> &str;
  fn set_message(&mut self, message: String);
}

pub struct AdapterStringTest {
  message: String,
}

impl AdapterStringTest {
  pub fn new() -> Self {
    Self {
      message: String::from("Hello, World!"),
    }
  }
}

#[async_trait]
impl AdapterPort<AdapterStringTest> for AdapterStringTest {
  fn token() -> &'static str {
    "ADAPTER_STRING_TEST"
  }

  async fn get_adapter(context: &Arc<dyn ContextPort>) -> Result<Box<Self>, Error> {
    let me = context
      .resolve_provider(Self::token())
      .await?
      .downcast::<Self>()
      .map_err(|_| format!("Cant resolve provider: {}", Self::token()))?;

    Ok(me)
  }
}

impl AdapterStringTestPort for AdapterStringTest {
  fn get_message(&self) -> &str {
    &self.message
  }

  fn set_message(&mut self, message: String) {
    self.message = message;
  }
}
