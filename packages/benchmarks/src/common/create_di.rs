use std::sync::Arc;

use ioc_container_rs::{
    adapters::{app_context::AppContext, di::DI},
    errors::error::Error,
};

use crate::adapters::{
    nested_adapter::{NestedAdapter, NestedPort},
    number_adapter::{NumberAdapter, NumberPort},
    string_adapter::{StringAdapter, StringPort},
};

pub async fn create_di() -> Result<Arc<AppContext>, Error> {
    DI::default()
        .bind::<dyn StringPort, StringAdapter>()
        .bind::<dyn NumberPort, NumberAdapter>()
        .bind::<dyn NestedPort, NestedAdapter>()
        .build()
        .await
}

pub fn create_graph() -> Arc<dyn NestedPort> {
    let string: Arc<dyn StringPort> = Arc::new(StringAdapter::default());
    let number: Arc<dyn NumberPort> = Arc::new(NumberAdapter::default());

    Arc::new(NestedAdapter::new(string, number))
}
