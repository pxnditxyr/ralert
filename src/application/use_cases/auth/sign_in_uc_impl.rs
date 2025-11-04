use async_trait::async_trait;
use std::sync::Arc;

use crate::{
    domain::{
        dtos::auth::sign_in_dto::{SignInDto, SignInResponseDto, UserResponseDto},
        entities::user_entity::UserStatus,
        errors::app_error::AppError,
        repositories::user_repository::UserRepository,
        use_cases::auth::sign_in_uc::SignInUseCase,
    },
    infrastructure::services::jwt_service::JwtService,
};

pub struct SignInUseCaseImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl SignInUseCaseImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl SignInUseCase for SignInUseCaseImpl {
    async fn execute(&self, dto: SignInDto) -> Result<SignInResponseDto, AppError> {
        let user = self
            .user_repository
            .find_by_email(&dto.email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("User is not active".to_string()))?;

        if user.status != UserStatus::Active {
            return Err(AppError::Unauthorized("User is not active".to_string()));
        }

        let is_valid = bcrypt::verify(&dto.password, &user.password).map_err(|e| {
            AppError::InternalServerError(format!("Password verification error: {e}"))
        })?;

        if !is_valid {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        let token = JwtService::generate_token(&user.id, &user.email, user.role.as_str())?;

        Ok(SignInResponseDto {
            token,
            user: UserResponseDto {
                id: user.id,
                email: user.email,
                name: user.name,
                role: user.role.to_string(),
            },
        })
    }
}
