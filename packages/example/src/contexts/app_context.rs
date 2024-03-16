use std::{any::Any, sync::Arc};

use ioc_container_rs::{container::container::Container, context::context::Context};
use tokio::sync::RwLock;

pub struct AppContext {
  context: Arc<RwLock<Container>>,
}

impl AppContext {
  pub fn new() -> Self {
    Self {
      context: Arc::new(RwLock::new(Container::new())),
    }
  }
}

impl Context for AppContext {
  async fn resolve_provider(&self, token: &'static str) -> Box<dyn Any> {
    let context = self.context.read().await;

    context
      .resolve(token)
      .await
      .expect(&format!("Cant resolve the {} provider", token))
  }

  async fn has_provider(&self, token: &'static str) -> bool {
    let context = self.context.read().await;

    context.has_provider(token).await
  }

  fn get_container(&self) -> Arc<RwLock<Container>> {
    self.context.clone()
  }
}
