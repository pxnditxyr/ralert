use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    domain::{
        dtos::health_status_dto::HealthStatusDto,
        errors::app_error::AppError
    },
    infrastructure::web::app_state::AppState,
};

pub async fn health_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HealthStatusDto>, AppError> {
    let health_status = state.health_check_use_case.execute().await?;
    Ok(Json(health_status))
}
