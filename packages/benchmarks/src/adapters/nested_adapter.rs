use std::sync::Arc;

use ioc_container_rs::{
    errors::error::Error,
    ports::{adapter_port::AdapterPort, context_port::ContextResolverPort},
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

impl<Context: ContextResolverPort> AdapterPort<dyn NestedPort, Context> for NestedAdapter {
    async fn construct(context: &Context) -> Result<Arc<dyn NestedPort>, Error> {
        let string = context.resolve::<dyn StringPort>()?;
        let number = context.resolve::<dyn NumberPort>()?;

        Ok(Arc::new(Self::new(string, number)))
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
