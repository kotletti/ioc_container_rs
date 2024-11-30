use std::sync::Arc;

use async_trait::async_trait;

use crate::{container::container::Container, errors::error::Error};

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
  async fn resolve_provider<T: 'static>(&self, token: &'static str) -> Result<Box<T>, Error> {
    let has_provider = self.has_provider(token).await;

    if has_provider == false {
      panic!("Provider {} has not exists in container", token.to_string());
    }

    self.container.resolve(token).await
  }

  async fn has_provider(&self, token: &'static str) -> bool {
    self.container.has_token(token).await
  }

  fn get_container(&self) -> Arc<Container> {
    self.container.clone()
  }
}
