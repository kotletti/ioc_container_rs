use crate::container::Container;

pub trait Context: Send + Sync {
  fn has_provider(&self, token: &'static str) -> bool;

  fn resolve_provider<Output: 'static>(&self, token: &'static str) -> Box<Output>;

  fn get_container(&self) -> &Container;
}
