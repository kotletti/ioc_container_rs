use async_trait::async_trait;

use crate::{
  entities::user_entity::UserEntity,
  ports::input::{
    add_user_port::AddUserPort, delete_user_port::DeleteUserPort, get_user_port::GetUserPort,
  },
};

#[async_trait]
pub trait UserServicePort {
  async fn add_user(&self, payload: &AddUserPort) -> Result<UserEntity, String>;
  async fn delete_user(&self, payload: &DeleteUserPort) -> Result<(), String>;
  async fn get_user(&self, payload: &GetUserPort) -> Result<Option<UserEntity>, String>;
  async fn get_count(&self) -> Result<usize, String>;
}
