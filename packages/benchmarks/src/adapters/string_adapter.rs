use std::sync::{Arc, RwLock};

use ioc_container_rs::{
    errors::error::Error,
    ports::{adapter_port::AdapterPort, context_port::ContextResolverPort},
};

pub trait StringPort: Send + Sync {
    fn get(&self) -> Result<String, Error>;
    fn set(&self, message: &str) -> Result<(), Error>;
}

pub struct StringAdapter {
    message: RwLock<String>,
}

impl Default for StringAdapter {
    fn default() -> Self {
        Self {
            message: RwLock::new(String::from("Hello, World!")),
        }
    }
}

impl<Context: ContextResolverPort> AdapterPort<dyn StringPort, Context> for StringAdapter {
    async fn construct(_: &Context) -> Result<Arc<dyn StringPort>, Error> {
        Ok(Arc::new(Self::default()))
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
