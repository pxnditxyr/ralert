use std::sync::Arc;

use sqlx::SqlitePool;

use crate::{
    application::use_cases::{
        auth::sign_in_uc_impl::SignInUseCaseImpl, health_check_uc_impl::HealthCheckUseCaseImpl,
        user::get_profile_uc_impl::GetProfileUseCaseImpl,
    },
    domain::{
        datasources::user_datasource::UserDatasource,
        repositories::user_repository::UserRepository,
        use_cases::{
            auth::sign_in_uc::SignInUseCase, health_check_uc::HealthCheckUseCase,
            user::get_profile_uc::GetProfileUseCase,
        },
    },
    infrastructure::{
        datasources::user_datasource_impl::UserDatasourceImpl,
        repositories::user_repository_impl::UserRepositoryImpl,
    },
};

pub struct Container {
    pub health_check_uc: Arc<dyn HealthCheckUseCase>,
    pub sign_in_uc: Arc<dyn SignInUseCase>,
    pub get_profile_uc: Arc<dyn GetProfileUseCase>,
}

impl Container {
    pub fn new(pool: SqlitePool) -> Self {
        let user_datasource: Arc<dyn UserDatasource> =
            Arc::new(UserDatasourceImpl::new(pool.clone()));
        let user_repository: Arc<dyn UserRepository> =
            Arc::new(UserRepositoryImpl::new(user_datasource));

        let health_check_uc = Arc::new(HealthCheckUseCaseImpl::new());
        let sign_in_uc = Arc::new(SignInUseCaseImpl::new(user_repository.clone()));
        let get_profile_uc = Arc::new(GetProfileUseCaseImpl::new(user_repository.clone()));

        Self {
            health_check_uc,
            sign_in_uc,
            get_profile_uc,
        }
    }
}
