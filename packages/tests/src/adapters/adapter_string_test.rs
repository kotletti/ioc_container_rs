pub struct AdapterStringTest {
  message: String,
}

impl AdapterStringTest {
  pub fn new() -> Self {
    Self {
      message: "Hello, World!".to_string(),
    }
  }

  pub fn token() -> &'static str {
    "ADAPTER_STRING_TEST"
  }

  pub fn get_message(&self) -> &str {
    &self.message
  }

  pub fn set_message(&mut self, message: String) {
    self.message = message;
  }
}
