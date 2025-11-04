use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    domain::errors::app_error::AppError, infrastructure::config::environments::ENVIRONMENTS,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

pub struct JwtService;

impl JwtService {
    pub fn generate_token(user_id: &str, email: &str, role: &str) -> Result<String, AppError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AppError::InternalServerError(format!("System time error: {e}")))?
            .as_secs() as i64;
        let expiration = now + (24 * 60 * 60);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            exp: expiration,
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(ENVIRONMENTS.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("Error generating token: {e}")))
    }

    pub fn verify_token(token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(ENVIRONMENTS.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {e}")))
    }
}
