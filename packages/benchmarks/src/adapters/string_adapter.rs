use std::sync::{Arc, RwLock};

use ioc_container_rs::{
    errors::error::Error,
    ports::{adapter_port::AdapterPort, container_port::ContainerPort},
};

pub trait StringPort: Send + Sync {
    fn get(&self) -> Result<String, Error>;
    fn set(&self, message: &str) -> Result<(), Error>;
}

pub struct StringAdapter {
    message: RwLock<String>,
}

impl StringAdapter {
    pub fn new() -> Self {
        Self {
            message: RwLock::new(String::from("Hello, World!")),
        }
    }
}

impl Default for StringAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterPort for dyn StringPort {
    fn construct(_: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
        Ok(Arc::new(StringAdapter::new()))
    }
}

impl StringPort for StringAdapter {
    fn get(&self) -> Result<String, Error> {
        let message = self
            .message
            .read()
            .map_err(|e| Error::from(e.to_string()))?;

        Ok(message.clone())
    }

    fn set(&self, message: &str) -> Result<(), Error> {
        let mut current = self
            .message
            .write()
            .map_err(|e| Error::from(e.to_string()))?;

        *current = message.to_string();

        Ok(())
    }
}
