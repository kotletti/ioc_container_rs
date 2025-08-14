use std::{any::Any, sync::Arc};

use async_trait::async_trait;

use crate::{
    container::container::Container, errors::error::Error, ports::context_port::ContextPort,
};

#[derive(Clone)]
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
impl ContextPort for ContainerContext {
    async fn resolve_provider(&self, token: &'static str) -> Result<Box<dyn Any>, Error> {
        let has_provider = self.has_provider(token).await;

        if has_provider == false {
            return Err(
                format!("Provider {} has not exists in container", token.to_string()).into(),
            );
        }

        self.container.resolve(token).await
    }

    async fn has_provider(&self, token: &'static str) -> bool {
        self.container.has_token(token).await
    }

    fn get_container(&self) -> Arc<Container> {
        self.container.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
