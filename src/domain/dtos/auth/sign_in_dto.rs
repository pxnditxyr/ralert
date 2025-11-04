use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SignInDto {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct SignInResponseDto {
    pub token: String,
    pub user: UserResponseDto,
}

#[derive(Debug, Serialize)]
pub struct UserResponseDto {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
}
