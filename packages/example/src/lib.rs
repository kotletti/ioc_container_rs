use std::{error::Error, sync::Arc};

use common::logger::logger::LoggerService;
use ioc_container_rs::container::Container;
use services::{
  logger_service::logger_service::ConsoleLoggerService, user_service::user_service::UserService,
};
use states::user_state::User;

mod common;
mod services;
mod states;

pub fn get_user_by_name_test(user_name: &str) -> Result<User, Box<dyn Error>> {
  let container = Container::new();

  container.register("LoggerService", || {
    let logger: Arc<dyn LoggerService> = Arc::new(ConsoleLoggerService::new("User"));
    Box::new(logger)
  });

  container.register("UserService", || {
    let logger: Arc<dyn LoggerService> = Arc::new(ConsoleLoggerService::new("User"));
    Box::new(UserService::new(logger))
  });

  let user_service: Box<UserService> = container.resolve("UserService").unwrap();

  user_service.get_user_by_name(&user_name)
}

#[cfg(test)]
mod tests {
  use crate::get_user_by_name_test;

  #[test]
  fn should_get_user_by_name() {
    let user_name = "Andrey";

    let user = get_user_by_name_test(user_name).unwrap();

    assert_eq!(user.get_name(), user_name);
  }
}
