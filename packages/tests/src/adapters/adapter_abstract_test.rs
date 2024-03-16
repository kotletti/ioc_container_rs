pub trait AdapterAbstractTestPort: Send + Sync + 'static {
  fn get_number(&self) -> i32;
  fn set_number(&mut self, number: i32);
}

pub struct AdapterAbstractTest {
  number: i32,
}

impl AdapterAbstractTest {
  pub fn new() -> Self {
    Self { number: 0 }
  }

  pub fn token() -> &'static str {
    "ADAPTER_ABSTRACT_TEST"
  }
}

impl AdapterAbstractTestPort for AdapterAbstractTest {
  fn get_number(&self) -> i32 {
    self.number
  }

  fn set_number(&mut self, number: i32) {
    self.number = number;
  }
}
