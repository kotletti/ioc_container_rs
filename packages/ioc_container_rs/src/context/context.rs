use std::sync::Arc;

use async_trait::async_trait;

use crate::container::container::Container;

#[async_trait]
pub trait Context: Send + Sync {
  async fn has_provider(&self, token: &'static str) -> bool;

  async fn resolve_provider<P: 'static>(&self, token: &'static str) -> P;

  fn get_container(&self) -> Arc<Container>;
}
