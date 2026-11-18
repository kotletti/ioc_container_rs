use std::sync::{Arc, RwLock};

use ioc_container_rs::{
    adapters::{app_context::AppContext, di::DI},
    errors::error::Error,
    ports::adapter_port::AdapterPort,
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

impl AdapterPort<dyn StringPort> for StringAdapter {
    fn construct(_: &Arc<AppContext>) -> Result<Arc<dyn StringPort>, Error> {
        Ok(Arc::new(StringAdapter::new()))
    }

    async fn create_di(di: DI) -> Result<DI, Error> {
        di.bind::<dyn StringPort, Self>()
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
