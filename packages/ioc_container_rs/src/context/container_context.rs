use std::sync::Arc;

use async_trait::async_trait;

use crate::container::container::Container;

use super::context::Context;

pub struct ContainerContext {
  container: Arc<Container>,
}

impl ContainerContext {
  pub fn new() -> Self {
    Self {
      container: Arc::new(Container::new()),
    }
  }
}

#[async_trait]
impl Context for ContainerContext {
  async fn resolve_provider<P: 'static>(&self, token: &'static str) -> P {
    let has_provider = self.has_provider(token).await;

    if has_provider == false {
      panic!("Provider {} has not exists in container", token.to_string());
    }

    self
      .container
      .resolve(token)
      .await
      .expect(&format!("Cant resolve the {} provider", &token))
  }

  async fn has_provider(&self, token: &'static str) -> bool {
    self.container.has_provider(token).await
  }

  fn get_container(&self) -> Arc<Container> {
    self.container.clone()
  }
}
