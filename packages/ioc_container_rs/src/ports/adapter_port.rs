use std::sync::Arc;

use async_trait::async_trait;

use crate::errors::error::Error;

use super::context_port::ContextPort;

#[async_trait]
pub trait AdapterPort<T>: Send + Sync + 'static {
  fn token() -> &'static str
  where
    Self: Sized;

  async fn get_adapter(context: &Arc<dyn ContextPort>) -> Result<Box<Self>, Error>;
}
