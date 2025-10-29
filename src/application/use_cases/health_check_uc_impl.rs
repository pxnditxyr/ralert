use async_trait::async_trait;

use crate::domain::{
    dtos::health_status_dto::HealthStatusDto,
    errors::app_error::AppError,
    use_cases::health_check_uc::HealthCheckUseCase
};

pub struct HealthCheckUseCaseImpl;


impl HealthCheckUseCaseImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl HealthCheckUseCase for HealthCheckUseCaseImpl {
    async fn execute(&self) -> Result<HealthStatusDto, AppError> {
        let status = HealthStatusDto {
            status: "Ok".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };
        Ok(status)
    }
}
