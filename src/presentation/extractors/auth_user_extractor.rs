use axum::{extract::FromRequestParts, http::request::Parts};

use crate::domain::{entities::auth_user_entity::AuthUserEntity, errors::app_error::AppError};

#[derive(Debug, Clone)]
pub struct AuthUser(pub AuthUserEntity);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_user = parts
            .extensions
            .get::<AuthUserEntity>()
            .cloned()
            .ok_or_else(|| {
                AppError::Unauthorized(
                    "Authentication required. User not found in request extensions".to_string(),
                )
            })?;
        Ok(AuthUser(auth_user))
    }
}

impl std::ops::Deref for AuthUser {
    type Target = AuthUserEntity;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
