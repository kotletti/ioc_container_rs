use std::sync::{Arc, RwLock};

use ioc_container_rs::{
    errors::error::Error,
    ports::{adapter_port::AdapterPort, container_port::ContainerPort},
};

pub trait NumberPort: Send + Sync {
    fn get(&self) -> Result<i32, Error>;
    fn set(&self, number: i32) -> Result<(), Error>;
}

pub struct NumberAdapter {
    number: RwLock<i32>,
}

impl NumberAdapter {
    pub fn new() -> Self {
        Self {
            number: RwLock::new(0),
        }
    }
}

impl Default for NumberAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl AdapterPort for dyn NumberPort {
    fn construct(_: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
        Ok(Arc::new(NumberAdapter::new()))
    }
}

impl NumberPort for NumberAdapter {
    fn get(&self) -> Result<i32, Error> {
        let number = self.number.read().map_err(|e| Error::from(e.to_string()))?;

        Ok(*number)
    }

    fn set(&self, number: i32) -> Result<(), Error> {
        let mut current = self
            .number
            .write()
            .map_err(|e| Error::from(e.to_string()))?;

        *current = number;

        Ok(())
    }
}
