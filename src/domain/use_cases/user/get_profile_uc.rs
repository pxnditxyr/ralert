use async_trait::async_trait;

use crate::domain::{dtos::users::user_profile_dto::UserProfileDto, errors::app_error::AppError};

#[async_trait]
pub trait GetProfileUseCase: Send + Sync {
    async fn execute(&self, user_id: &str) -> Result<UserProfileDto, AppError>;
}
