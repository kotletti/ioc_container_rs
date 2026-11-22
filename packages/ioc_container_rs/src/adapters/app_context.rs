use std::{any::TypeId, collections::HashMap};

use crate::{
    adapters::container::Container,
    errors::error::Error,
    ports::{
        container_port::{ContainerPort, ContainerValue},
        context_port::{ContextReaderPort, ContextWriterPort},
    },
};

pub struct AppContext(Box<dyn ContainerPort>);

impl Default for AppContext {
    fn default() -> Self {
        Self(Box::new(Container::create(HashMap::new())))
    }
}

impl ContextReaderPort for AppContext {
    fn read(&self, key: TypeId, name: &'static str) -> Result<ContainerValue, Error> {
        self.0.get(key, name)
    }
}

impl ContextWriterPort for AppContext {
    fn write(&mut self, key: TypeId, value: ContainerValue) -> Result<(), Error> {
        self.0.insert(key, value);

        Ok(())
    }
}
