use std::sync::Arc;

use ioc_container_rs::{
    adapters::{app_context::AppContext, di::DI},
    errors::error::Error,
    ports::container_port::ContainerPort,
};

use crate::adapters::{
    nested_adapter::{NestedAdapter, NestedPort},
    number_adapter::{NumberAdapter, NumberPort},
    string_adapter::{StringAdapter, StringPort},
};

pub fn create_di() -> Result<Arc<dyn ContainerPort>, Error> {
    let container = DI::new()
        .bind::<dyn StringPort>()?
        .bind::<dyn NumberPort>()?
        .bind::<dyn NestedPort>()?
        .build::<AppContext>();

    Ok(container)
}

pub fn create_graph() -> Arc<dyn NestedPort> {
    let string: Arc<dyn StringPort> = Arc::new(StringAdapter::new());
    let number: Arc<dyn NumberPort> = Arc::new(NumberAdapter::new());

    Arc::new(NestedAdapter::new(string, number))
}
