use std::sync::Arc;

use async_trait::async_trait;
use ioc_container_rs::{
    errors::error::Error,
    ports::{adapter_port::AdapterPort, context_port::ContextPort},
};

use super::{
    adapter_number_test::{AdapterNumberTest, AdapterNumberTestPort},
    adapter_string_test::{AdapterStringTest, AdapterStringTestPort},
};

#[async_trait]
pub trait AdapterNestedPort: Sync + Send + 'static {
    async fn set_number(&self, number: i32) -> Result<(), Error>;
    async fn set_string(&self, message: &str) -> Result<(), Error>;
    async fn get_string(&self) -> Result<String, Error>;
    async fn get_number(&self) -> Result<i32, Error>;
}

pub struct AdapterNested {
    context: Arc<dyn ContextPort>,
}

impl AdapterNested {
    pub fn new(context: Arc<dyn ContextPort>) -> Self {
        Self { context }
    }
}

#[async_trait]
impl AdapterPort<AdapterNested> for AdapterNested {
    fn token() -> &'static str {
        "ADAPTER_NESTED_TEST"
    }
}

#[async_trait]
impl AdapterNestedPort for AdapterNested {
    async fn set_number(&self, number: i32) -> Result<(), Error> {
        let mut svc = AdapterNumberTest::get_adapter(&self.context).await?;

        svc.set_number(number);

        Ok(())
    }

    async fn set_string(&self, message: &str) -> Result<(), Error> {
        let mut svc = AdapterStringTest::get_adapter(&self.context).await?;

        svc.set_message(message.to_string());

        Ok(())
    }

    async fn get_string(&self) -> Result<String, Error> {
        let svc = AdapterStringTest::get_adapter(&self.context).await?;

        let message = svc.get_message().to_string();

        Ok(message)
    }

    async fn get_number(&self) -> Result<i32, Error> {
        let svc = AdapterNumberTest::get_adapter(&self.context).await?;

        let number = svc.get_number();

        Ok(number)
    }
}
