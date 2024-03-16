use async_trait::async_trait;

use crate::entities::user_entity::UserEntity;

#[async_trait]
pub trait UserRepositoryPort {
  async fn add_user(&self, entity: &UserEntity) -> Result<(), String>;
  async fn delete_user(&self, entity: &UserEntity) -> Result<(), String>;
  async fn get_user(&self, entity: &UserEntity) -> Result<Option<UserEntity>, String>;
  async fn get_user_by_email(&self, email: &str) -> Result<Option<UserEntity>, String>;
  async fn get_count(&self) -> Result<usize, String>;
}
