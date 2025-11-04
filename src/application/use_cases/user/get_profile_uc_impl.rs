use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::{
    dtos::users::user_profile_dto::UserProfileDto, errors::app_error::AppError,
    repositories::user_repository::UserRepository,
    use_cases::user::get_profile_uc::GetProfileUseCase,
};

pub struct GetProfileUseCaseImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl GetProfileUseCaseImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl GetProfileUseCase for GetProfileUseCaseImpl {
    async fn execute(&self, user_id: &str) -> Result<UserProfileDto, AppError> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserProfileDto {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role.to_string(),
        })
    }
}
