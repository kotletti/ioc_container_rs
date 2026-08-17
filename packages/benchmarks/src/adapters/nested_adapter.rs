use std::sync::Arc;

use ioc_container_rs::{
    errors::error::Error,
    ports::{adapter_port::AdapterPort, container_port::ContainerPort},
};

use crate::adapters::{number_adapter::NumberPort, string_adapter::StringPort};

pub trait NestedPort: Send + Sync {
    fn get_string(&self) -> Result<String, Error>;
    fn set_string(&self, message: &str) -> Result<(), Error>;
    fn get_number(&self) -> Result<i32, Error>;
    fn set_number(&self, number: i32) -> Result<(), Error>;
}

pub struct NestedAdapter {
    string: Arc<dyn StringPort>,
    number: Arc<dyn NumberPort>,
}

impl NestedAdapter {
    pub fn new(string: Arc<dyn StringPort>, number: Arc<dyn NumberPort>) -> Self {
        Self { string, number }
    }
}

impl AdapterPort for dyn NestedPort {
    fn construct(container: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
        let string = <dyn StringPort>::get_adapter(container)?;
        let number = <dyn NumberPort>::get_adapter(container)?;

        Ok(Arc::new(NestedAdapter::new(string, number)))
    }
}

impl NestedPort for NestedAdapter {
    fn get_string(&self) -> Result<String, Error> {
        self.string.get()
    }

    fn set_string(&self, message: &str) -> Result<(), Error> {
        self.string.set(message)
    }

    fn get_number(&self) -> Result<i32, Error> {
        self.number.get()
    }

    fn set_number(&self, number: i32) -> Result<(), Error> {
        self.number.set(number)
    }
}
