use crate::errors::error::Error;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    pin::Pin,
    sync::{Arc, OnceLock},
};

pub type BoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub type ContainerValue = Arc<dyn Any + Send + Sync>;

pub type ContainerFactory<Context> =
    Box<dyn for<'a> FnOnce(&'a Context) -> BoxedFuture<'a, Result<ContainerValue, Error>> + Send>;

pub type ContainerStorage = HashMap<TypeId, ContainerValue>;

pub type ContainerCache = OnceLock<Arc<dyn Any + Send + Sync>>;

pub trait ContainerPort: Send + Sync + 'static {
    fn create(storage: ContainerStorage) -> Self
    where
        Self: Sized;

    fn insert(&mut self, key: TypeId, value: ContainerValue);

    fn get(&self, key: TypeId, name: &'static str) -> Result<ContainerValue, Error>;

    fn existed(&self, key: TypeId) -> bool;
}
