use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use crate::{
    adapters::container::Container,
    errors::error::Error,
    ports::container_port::{ContainerPort, ContainerStorage},
};

pub struct AppContext {
    context: Arc<dyn ContainerPort>,
}

impl AppContext {
    pub fn new(context: Arc<dyn ContainerPort>) -> Self {
        Self { context }
    }

    pub fn context(&self) -> Arc<dyn ContainerPort> {
        self.context.clone()
    }
}

impl ContainerPort for AppContext {
    fn create(storage: ContainerStorage) -> Arc<Self> {
        Arc::new(Self {
            context: Container::create(storage),
        })
    }

    fn resolve(
        self: Arc<Self>,
        key: TypeId,
        name: &'static str,
    ) -> Result<Arc<dyn Any + Send + Sync>, Error> {
        self.context.clone().resolve(key, name)
    }

    fn existed(&self, key: TypeId) -> bool {
        self.context.existed(key)
    }
}
