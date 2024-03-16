pub struct AdapterNumberTest {
  number: i32,
}

impl AdapterNumberTest {
  pub fn new() -> Self {
    Self { number: 0 }
  }

  pub fn token() -> &'static str {
    "ADAPTER_NUMBER_TEST"
  }

  pub fn get_number(&self) -> i32 {
    self.number
  }

  pub fn set_number(&mut self, number: i32) {
    self.number = number;
  }
}
