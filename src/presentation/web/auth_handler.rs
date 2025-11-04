use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    domain::{
        dtos::auth::sign_in_dto::{SignInDto, SignInResponseDto},
        errors::app_error::AppError,
    },
    infrastructure::web::app_state::AppState,
};

pub async fn sign_in_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SignInDto>,
) -> Result<Json<SignInResponseDto>, AppError> {
    let response = state.sign_in_use_case.execute(dto).await?;
    Ok(Json(response))
}
