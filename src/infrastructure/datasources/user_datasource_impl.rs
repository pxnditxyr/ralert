use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::domain::{
    datasources::user_datasource::UserDatasource, dtos::users::user_row_dto::UserRowDto,
    errors::app_error::AppError,
};

pub struct UserDatasourceImpl {
    pool: SqlitePool,
}

impl UserDatasourceImpl {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserDatasource for UserDatasourceImpl {
    async fn find_by_email(&self, email: &str) -> Result<Option<UserRowDto>, AppError> {
        let result = sqlx::query_as::<_, SqliteUserRow>(
            "SELECT id, email, password, name, role, status, created_at, updated_at
            FROM users
            WHERE email = ?",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Database error: {e}")))?;

        Ok(result.map(|row| UserRowDto {
            id: row.id,
            email: row.email,
            password: row.password,
            name: row.name,
            role: row.role,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }
    async fn find_by_id(&self, id: &str) -> Result<Option<UserRowDto>, AppError> {
        let result = sqlx::query_as::<_, SqliteUserRow>(
            "SELECT id, email, password, name, role, status, created_at, updated_at
            FROM users
            WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Database error: {e}")))?;

        Ok(result.map(|row| UserRowDto {
            id: row.id,
            email: row.email,
            password: row.password,
            name: row.name,
            role: row.role,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }
}

#[derive(sqlx::FromRow)]
struct SqliteUserRow {
    id: String,
    email: String,
    password: String,
    name: String,
    role: String,
    status: String,
    created_at: i64,
    updated_at: i64,
}
