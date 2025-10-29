use serde::Serialize;

#[derive(Serialize)]
pub struct HealthStatusDto {
    pub status: String,
    pub version: String,
}
