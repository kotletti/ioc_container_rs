pub trait LoggerService: Send + Sync {
  fn log(&self, message: &str);
}
