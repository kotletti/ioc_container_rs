use crate::container::Container;

use super::context::Context;

type Providers = Vec<&'static str>;

pub struct ContainerContext {
  container: Container,
  providers: Providers,
}

pub struct ContainerContextProps {
  pub providers: Providers,
  pub container: Container,
}

impl ContainerContext {
  pub fn new(props: ContainerContextProps) -> Self {
    ContainerContext {
      container: props.container,
      providers: props.providers,
    }
  }
}

impl Context for ContainerContext {
  fn resolve_provider<Output: 'static>(&self, token: &'static str) -> Box<Output> {
    if self.has_provider(token) == false {
      panic!("Cant resolve the {} provider", token);
    }

    self.container.resolve(token).unwrap()
  }

  fn has_provider(&self, token: &'static str) -> bool {
    self.providers.contains(&token)
  }

  fn get_container(&self) -> &Container {
    &self.container
  }
}
