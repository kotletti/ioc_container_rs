use async_trait::async_trait;
use ioc_container_rs::errors::error::Error;

use crate::entities::user_entity::UserEntity;

#[async_trait]
pub trait UserServicePort: Send + Sync {
    async fn add_user(&self, id: u32, name: &str, email: &str) -> Result<UserEntity, Error>;
    async fn delete_user(&self, id: u32) -> Result<(), Error>;
    async fn get_user(&self, id: u32) -> Result<UserEntity, Error>;
    async fn get_count(&self) -> Result<usize, Error>;
}
