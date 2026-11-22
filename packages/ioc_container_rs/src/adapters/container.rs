use crate::{
    errors::error::Error,
    ports::container_port::{ContainerPort, ContainerStorage, ContainerValue},
};
use std::any::TypeId;

pub struct Container {
    storage: ContainerStorage,
}

impl ContainerPort for Container {
    fn create(storage: ContainerStorage) -> Self {
        Self { storage }
    }

    fn get(&self, key: TypeId, name: &'static str) -> Result<ContainerValue, Error> {
        self.storage
            .get(&key)
            .cloned()
            .ok_or_else(|| Error::from(format!("Adapter {name} is not built")))
    }

    fn insert(&mut self, key: TypeId, value: ContainerValue) {
        self.storage.insert(key, value);
    }

    fn existed(&self, key: TypeId) -> bool {
        self.storage.contains_key(&key)
    }
}
