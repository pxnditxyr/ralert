use std::sync::Arc;

use crate::domain::use_cases::health_check_uc::HealthCheckUseCase;


pub struct AppState {
    pub health_check_use_case: Arc<dyn HealthCheckUseCase>,
}
