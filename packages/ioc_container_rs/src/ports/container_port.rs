use crate::errors::error::Error;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

pub type ContainerFactoryItem =
    Box<dyn Fn(Arc<dyn ContainerPort>) -> Result<Arc<dyn Any + Send + Sync>, Error> + Send + Sync>;

pub type ContainerStorage = HashMap<TypeId, ContainerFactoryItem>;

pub trait ContainerPort: Send + Sync + 'static {
    fn create(storage: ContainerStorage) -> Arc<Self>
    where
        Self: Sized;

    fn resolve(
        self: Arc<Self>,
        key: TypeId,
        name: &'static str,
    ) -> Result<Arc<dyn Any + Send + Sync>, Error>;
    fn existed(&self, key: TypeId) -> bool;
}
