use axum::{extract::Request, http::HeaderMap, middleware::Next, response::Response};

use crate::{
    domain::{entities::auth_user_entity::AuthUserEntity, errors::app_error::AppError},
    infrastructure::services::jwt_service::JwtService,
};

pub async fn jwt_auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_token_from_headers(&headers)?;
    let claims = JwtService::verify_token(token)?;

    let auth_user = AuthUserEntity {
        id: claims.sub,
        email: claims.email,
        role: claims.role,
    };

    request.extensions_mut().insert(auth_user);
    Ok(next.run(request).await)
}

pub async fn admin_only_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_token_from_headers(&headers)?;
    let claims = JwtService::verify_token(token)?;

    if claims.role != "admin" {
        return Err(AppError::Unauthorized("Admin access required".to_string()));
    }

    let auth_user = AuthUserEntity {
        id: claims.sub,
        email: claims.email,
        role: claims.role,
    };

    request.extensions_mut().insert(auth_user);
    Ok(next.run(request).await)
}

fn extract_token_from_headers(headers: &HeaderMap) -> Result<&str, AppError> {
    let auth_header = headers
        .get("Authorization")
        .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| AppError::Unauthorized("Invalid Authorization header".to_string()))?;

    if !auth_str.starts_with("Bearer ") {
        return Err(AppError::Unauthorized(
            "Invalid Authorization format. Expected 'Bearer <token>'".to_string(),
        ));
    }

    let token = auth_str.trim_start_matches("Bearer ").trim();

    if token.is_empty() {
        return Err(AppError::Unauthorized("Empty token".to_string()));
    }

    Ok(token)
}
