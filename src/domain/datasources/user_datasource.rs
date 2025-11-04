use async_trait::async_trait;

use crate::domain::{dtos::users::user_row_dto::UserRowDto, errors::app_error::AppError};

#[async_trait]
pub trait UserDatasource: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<UserRowDto>, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<UserRowDto>, AppError>;
}
