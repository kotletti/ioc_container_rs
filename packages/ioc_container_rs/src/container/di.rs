use std::sync::Arc;

use crate::{
  errors::error::Error,
  ports::{adapter_port::AdapterPort, context_port::ContextPort},
};

pub struct InjectAdapter<T> {
  pub token: &'static str,
  pub factory: Arc<dyn Fn(Arc<dyn ContextPort>) -> T + Send + Sync + 'static>,
}

#[derive(Clone)]
pub struct DI {
  context: Arc<dyn ContextPort>,
}

impl DI {
  pub fn new(context: Arc<dyn ContextPort>) -> Self {
    Self { context }
  }

  pub async fn inject<T: AdapterPort<T>>(&self, injector: InjectAdapter<T>) -> Result<Self, Error> {
    let container = self.context.get_container();
    let factory = injector.factory;
    let context = self.context.clone();

    container
      .register(injector.token, move || factory(context.clone()))
      .await?;

    Ok(self.clone())
  }

  pub fn get_context(&self) -> Arc<dyn ContextPort> {
    self.context.clone()
  }
}
