use std::{any::Any, collections::HashMap, sync::Arc};

use async_trait::async_trait;
use ioc_container_rs::{
    container::container::Container, errors::error::Error, ports::context_port::ContextPort,
};
use tokio::sync::RwLock;

pub struct CustomContext {
    container: Arc<Container>,
    metrics: Arc<RwLock<HashMap<String, usize>>>,
}

impl CustomContext {
    pub fn new() -> Self {
        Self {
            container: Arc::new(Container::new()),
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_metrics(&self) -> Arc<RwLock<HashMap<String, usize>>> {
        self.metrics.clone()
    }

    async fn increment_metric(&self, token: &'static str) {
        let mut metrics = self.metrics.write().await;

        let count = metrics.entry(token.to_string()).or_insert(0);

        if *count == 0 {
            *count = 1;
        } else {
            *count += 1;
        }
    }
}

#[async_trait]
impl ContextPort for CustomContext {
    async fn resolve_provider(&self, token: &'static str) -> Result<Box<dyn Any>, Error> {
        let has_provider = self.has_provider(token).await;

        if has_provider == false {
            return Err(
                format!("Provider {} has not exists in container", token.to_string()).into(),
            );
        }

        self.increment_metric(token).await;

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
