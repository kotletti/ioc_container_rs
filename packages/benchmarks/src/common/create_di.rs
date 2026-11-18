use std::sync::Arc;

use ioc_container_rs::{
    adapters::{app_context::AppContext, di::DI},
    errors::error::Error,
    ports::adapter_port::AdapterPort,
};

use crate::adapters::{
    nested_adapter::{NestedAdapter, NestedPort},
    number_adapter::{NumberAdapter, NumberPort},
    string_adapter::{StringAdapter, StringPort},
};

pub async fn create_di() -> Result<Arc<AppContext>, Error> {
    let di = DI::new();

    let di = StringAdapter::create_di(di).await?;
    let di = NumberAdapter::create_di(di).await?;
    let di = NestedAdapter::create_di(di).await?;

    Ok(di.build::<AppContext>())
}

pub fn create_graph() -> Arc<dyn NestedPort> {
    let string: Arc<dyn StringPort> = Arc::new(StringAdapter::new());
    let number: Arc<dyn NumberPort> = Arc::new(NumberAdapter::new());

    Arc::new(NestedAdapter::new(string, number))
}
