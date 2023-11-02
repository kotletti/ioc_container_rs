use crate::common::logger::logger::LoggerService;

pub struct ConsoleLoggerService {
  prefix: String,
}

impl ConsoleLoggerService {
  pub fn new(prefix: &str) -> Self {
    Self {
      prefix: prefix.to_string(),
    }
  }
}

impl LoggerService for ConsoleLoggerService {
  fn log(&self, message: &str) {
    println!("{}: {}", self.prefix, message);
  }
}
