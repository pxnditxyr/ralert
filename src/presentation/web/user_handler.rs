use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    domain::{dtos::users::user_profile_dto::UserProfileDto, errors::app_error::AppError},
    infrastructure::web::app_state::AppState,
    presentation::extractors::auth_user_extractor::AuthUser,
};

pub async fn get_profile_handler(
    State(state): State<Arc<AppState>>,
    auth_user: AuthUser,
) -> Result<Json<UserProfileDto>, AppError> {
    let profile = state
        .get_user_profile_use_case
        .execute(&auth_user.id)
        .await?;
    Ok(Json(profile))
}

pub async fn admin_only_handler(auth_user: AuthUser) -> Result<Json<serde_json::Value>, AppError> {
    if !auth_user.is_admin() {
        return Err(AppError::Unauthorized("Admin access required".to_string()));
    }

    Ok(Json(serde_json::json!({
        "message": "Admin access granted",
        "user": {
            "id": auth_user.id,
            "email": auth_user.email,
            "role": auth_user.role,
        }
    })))
}
