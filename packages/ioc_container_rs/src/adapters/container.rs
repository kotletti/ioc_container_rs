use crate::{
    errors::error::Error,
    ports::container_port::{ContainerPort, ContainerSlot, ContainerStorage},
};
use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, OnceLock},
};

pub struct Container {
    slots: HashMap<TypeId, Arc<ContainerSlot>>,
}

impl ContainerPort for Container {
    fn create(storage: ContainerStorage) -> Arc<Self>
    where
        Self: Sized,
    {
        let slots = storage
            .into_iter()
            .map(|(token, factory)| {
                (
                    token,
                    Arc::new(ContainerSlot::new(factory, OnceLock::new())),
                )
            })
            .collect();

        Arc::new(Self { slots })
    }

    fn get(self: Arc<Self>, key: TypeId, name: &'static str) -> Result<Arc<ContainerSlot>, Error> {
        let slot = self
            .slots
            .get(&key)
            .ok_or_else(|| Error::from(format!("Adapter {name} has not exists in container")))?;

        // if let Some(instance) = slot.cache.get() {
        //     return Ok(instance.clone());
        // }

        // let built = (slot.factory)(self.clone());
        // let instance = built?;

        // Ok(slot.cache.get_or_init(|| instance).clone())

        Ok(slot.clone())
    }

    fn existed(&self, key: TypeId) -> bool {
        self.slots.contains_key(&key)
    }
}
