use async_trait::async_trait;

use crate::domain::{
    dtos::auth::sign_in_dto::{SignInDto, SignInResponseDto},
    errors::app_error::AppError,
};

#[async_trait]
pub trait SignInUseCase: Send + Sync {
    async fn execute(&self, dto: SignInDto) -> Result<SignInResponseDto, AppError>;
}
