use std::sync::Arc;

use async_trait::async_trait;

use crate::{container::container::Container, errors::error::Error};

#[async_trait]
pub trait Context: Send + Sync {
  async fn has_provider(&self, token: &'static str) -> bool;

  async fn resolve_provider<T: 'static>(&self, token: &'static str) -> Result<Box<T>, Error>;

  fn get_container(&self) -> Arc<Container>;
}
