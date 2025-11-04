use std::sync::Arc;

use crate::domain::use_cases::{
    auth::sign_in_uc::SignInUseCase, health_check_uc::HealthCheckUseCase,
    user::get_profile_uc::GetProfileUseCase,
};

pub struct AppState {
    pub health_check_use_case: Arc<dyn HealthCheckUseCase>,
    pub sign_in_use_case: Arc<dyn SignInUseCase>,
    pub get_user_profile_use_case: Arc<dyn GetProfileUseCase>,
}
