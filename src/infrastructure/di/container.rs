use std::sync::Arc;

use crate::{
    application::use_cases::health_check_uc_impl::HealthCheckUseCaseImpl,
    domain::use_cases::health_check_uc::HealthCheckUseCase
};


pub struct Container {
    pub health_check_uc: Arc<dyn HealthCheckUseCase>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            health_check_uc: Arc::new(HealthCheckUseCaseImpl::new()),
        }
    }
}
