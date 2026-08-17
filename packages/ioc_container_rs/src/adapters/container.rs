use crate::{
    errors::error::Error,
    ports::container_port::{ContainerFactoryItem, ContainerPort, ContainerStorage},
};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock},
};

struct Slot {
    factory: ContainerFactoryItem,
    cache: OnceLock<Arc<dyn Any + Send + Sync>>,
}

pub struct Container {
    slots: HashMap<TypeId, Slot>,
}

impl Container {
    pub fn new(storage: ContainerStorage) -> Arc<Self> {
        let slots = storage
            .into_iter()
            .map(|(token, factory)| {
                (
                    token,
                    Slot {
                        factory,
                        cache: OnceLock::new(),
                    },
                )
            })
            .collect();

        Arc::new(Self { slots })
    }
}

impl ContainerPort for Container {
    fn create(storage: ContainerStorage) -> Arc<Self>
    where
        Self: Sized,
    {
        Self::new(storage)
    }

    fn resolve(
        self: Arc<Self>,
        key: TypeId,
        name: &'static str,
    ) -> Result<Arc<dyn Any + Send + Sync>, Error> {
        let slot = self
            .slots
            .get(&key)
            .ok_or_else(|| Error::from(format!("Adapter {name} has not exists in container")))?;

        if let Some(instance) = slot.cache.get() {
            return Ok(instance.clone());
        }

        let built = (slot.factory)(self.clone());
        let instance = built?;

        Ok(slot.cache.get_or_init(|| instance).clone())
    }

    fn existed(&self, key: TypeId) -> bool {
        self.slots.contains_key(&key)
    }
}
