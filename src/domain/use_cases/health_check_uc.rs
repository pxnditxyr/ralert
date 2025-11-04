use async_trait::async_trait;

use crate::domain::{dtos::health_status_dto::HealthStatusDto, errors::app_error::AppError};

#[async_trait]
pub trait HealthCheckUseCase: Send + Sync {
    async fn execute(&self) -> Result<HealthStatusDto, AppError>;
}
