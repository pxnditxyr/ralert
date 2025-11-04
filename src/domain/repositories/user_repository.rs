use async_trait::async_trait;

use crate::domain::{entities::user_entity::UserEntity, errors::app_error::AppError};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<UserEntity>, AppError>;
}
