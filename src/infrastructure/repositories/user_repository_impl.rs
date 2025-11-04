use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::mappers::user_mapper::UserMapper,
    domain::{
        datasources::user_datasource::UserDatasource, entities::user_entity::UserEntity,
        errors::app_error::AppError, repositories::user_repository::UserRepository,
    },
};

pub struct UserRepositoryImpl {
    datasource: Arc<dyn UserDatasource>,
}

impl UserRepositoryImpl {
    pub fn new(datasource: Arc<dyn UserDatasource>) -> Self {
        Self { datasource }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_email(&self, email: &str) -> Result<Option<UserEntity>, AppError> {
        let row_dto = self.datasource.find_by_email(email).await?;
        Ok(row_dto.map(UserMapper::row_to_entity))
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<UserEntity>, AppError> {
        let row_dto = self.datasource.find_by_id(id).await?;
        Ok(row_dto.map(UserMapper::row_to_entity))
    }
}
