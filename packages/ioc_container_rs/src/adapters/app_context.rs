use std::{
    any::{Any, TypeId, type_name},
    sync::Arc,
};

use crate::{
    adapters::container::Container,
    errors::error::Error,
    ports::{
        container_port::{ContainerPort, ContainerStorage},
        context_port::ContextPort,
    },
};

pub struct AppContext(Arc<dyn ContainerPort>);

impl AppContext {
    pub fn resolve<T: Send + Sync + 'static + ?Sized>(self: &Arc<Self>) -> Result<Arc<T>, Error> {
        let key = TypeId::of::<T>();
        let name = type_name::<T>();

        let slot = self.container().get(key, name)?;

        if let Some(instance) = slot.cache().get() {
            return self.downcast::<T>(instance.clone());
        }

        let factory = slot.factory();
        let any = factory(self.clone())?;

        let instance = slot.cache().get_or_init(|| any).clone();

        self.downcast::<T>(instance)
    }

    fn downcast<T: Send + Sync + ?Sized + 'static>(
        &self,
        any: Arc<dyn Any>,
    ) -> Result<Arc<T>, Error> {
        let name = type_name::<T>();

        let arc = any
            .downcast_ref::<Arc<T>>()
            .ok_or(format!("Cant downcast adapter {name}"))?;

        Ok(arc.clone())
    }

    fn container(&self) -> Arc<dyn ContainerPort> {
        self.0.clone()
    }
}

impl ContextPort for AppContext {
    type Storage = ContainerStorage;

    fn create(storage: Self::Storage) -> Arc<Self> {
        Arc::new(Self(Container::create(storage)))
    }
}
