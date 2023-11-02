use std::{error::Error, sync::Arc};

use crate::{
  common::logger::logger::LoggerService,
  states::user_state::{User, UserState},
};

pub struct UserService {
  logger: Arc<dyn LoggerService>,
  state: UserState,
}

impl UserService {
  pub fn new(logger: Arc<dyn LoggerService>) -> Self {
    let users = [
      User::new("Andrey", "andrey@email.domain"),
      User::new("Daria", "daria@email.domain"),
      User::new("Kirill", "kirill@email.domain"),
    ]
    .into();

    Self {
      logger,
      state: UserState::new(users),
    }
  }

  pub fn get_user_by_name(&self, name: &str) -> Result<User, Box<dyn Error>> {
    self.logger.log("Get user by name.");

    self.state.get_user_by_name(name)
  }
}
