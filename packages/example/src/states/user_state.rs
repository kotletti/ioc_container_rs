use std::error::Error;

#[derive(Clone)]
pub struct User {
  name: String,
  email: String,
}

impl User {
  pub fn new(name: &str, email: &str) -> Self {
    Self {
      name: name.to_string(),
      email: email.to_string(),
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn get_email(&self) -> &str {
    &self.email
  }
}

pub struct UserState {
  users: Vec<User>,
}

impl UserState {
  pub fn new(users: Vec<User>) -> Self {
    Self { users }
  }

  pub fn get_user_by_name(&self, name: &str) -> Result<User, Box<dyn Error>> {
    match &self.users.clone().into_iter().find(|i| i.name.eq(name)) {
      Some(u) => Ok(u.clone()),
      None => return Err("User not found".into()),
    }
  }
}
