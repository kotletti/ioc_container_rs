use std::{any::Any, sync::Arc};

use async_trait::async_trait;

use crate::{container::container::Container, errors::error::Error};

#[async_trait]
pub trait ContextPort: Send + Sync {
  async fn has_provider(&self, token: &'static str) -> bool;

  async fn resolve_provider(&self, token: &'static str) -> Result<Box<dyn Any>, Error>;

  fn get_container(&self) -> Arc<Container>;

  fn as_any(&self) -> &dyn Any;
}
