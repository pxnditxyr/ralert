use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUserEntity {
    pub id: String,
    pub email: String,
    pub role: String,
}

impl AuthUserEntity {
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
    pub fn is_user(&self) -> bool {
        self.role == "user"
    }
}
