use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserProfileDto {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
}
