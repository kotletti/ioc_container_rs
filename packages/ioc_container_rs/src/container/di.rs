use std::sync::Arc;

use crate::context::{container_context::ContainerContext, context::Context};

pub struct InjectAdapter<T> {
  pub token: &'static str,
  pub factory: Arc<dyn Fn(Arc<ContainerContext>) -> T + Send + Sync + 'static>,
}

#[derive(Clone)]
pub struct DI {
  context: Arc<ContainerContext>,
}

impl DI {
  pub fn new() -> Self {
    let context = Arc::new(ContainerContext::new());

    Self { context }
  }

  pub async fn inject<T: Send + Sync + 'static>(&self, injector: InjectAdapter<T>) -> Self {
    let container = self.context.get_container();
    let factory = injector.factory;
    let context = self.context.clone();

    container
      .register(injector.token, move || factory(context.clone()))
      .await;

    self.clone()
  }

  pub fn get_context(&self) -> Arc<ContainerContext> {
    self.context.clone()
  }
}
