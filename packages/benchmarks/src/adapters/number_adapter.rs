use std::sync::{Arc, RwLock};

use ioc_container_rs::{
    errors::error::Error,
    ports::{adapter_port::AdapterPort, context_port::ContextResolverPort},
};

pub trait NumberPort: Send + Sync {
    fn get(&self) -> Result<i32, Error>;
    fn set(&self, number: i32) -> Result<(), Error>;
}

pub struct NumberAdapter {
    number: RwLock<i32>,
}

impl Default for NumberAdapter {
    fn default() -> Self {
        Self {
            number: RwLock::new(0),
        }
    }
}

impl<Context: ContextResolverPort> AdapterPort<dyn NumberPort, Context> for NumberAdapter {
    async fn construct(_: &Context) -> Result<Arc<dyn NumberPort>, Error> {
        Ok(Arc::new(Self::default()))
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
