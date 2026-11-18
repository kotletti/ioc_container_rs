use crate::{adapters::app_context::AppContext, errors::error::Error};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, OnceLock},
};

pub type ContainerFactory =
    Box<dyn Fn(Arc<AppContext>) -> Result<Arc<dyn Any + Send + Sync>, Error> + Send + Sync>;

pub type ContainerStorage = HashMap<TypeId, ContainerFactory>;

pub type ContainerCache = OnceLock<Arc<dyn Any + Send + Sync>>;

pub struct ContainerSlot {
    factory: ContainerFactory,
    cache: ContainerCache,
}

impl ContainerSlot {
    pub fn new(factory: ContainerFactory, cache: ContainerCache) -> Self {
        Self { factory, cache }
    }

    pub fn factory(&self) -> &ContainerFactory {
        &self.factory
    }

    pub fn cache(&self) -> &ContainerCache {
        &self.cache
    }
}

pub trait ContainerPort: Send + Sync + 'static {
    fn create(storage: ContainerStorage) -> Arc<Self>
    where
        Self: Sized;

    fn get(self: Arc<Self>, key: TypeId, name: &'static str) -> Result<Arc<ContainerSlot>, Error>;

    fn existed(&self, key: TypeId) -> bool;
}
