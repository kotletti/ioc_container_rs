use std::sync::Arc;

use async_trait::async_trait;

use crate::{context::container_context::ContainerContext, errors::error::Error};

#[async_trait]
pub trait AdapterPort<T>: Send + Sync + 'static {
  fn token() -> &'static str
  where
    Self: Sized;

  async fn get_adapter(context: &Arc<ContainerContext>) -> Result<Box<Self>, Error>;
}
