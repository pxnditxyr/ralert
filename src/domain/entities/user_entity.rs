use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Inactive,
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
        }
    }
}

impl UserStatus {
    pub fn as_str(&self) -> &str {
        match self {
            UserStatus::Active => "active",
            UserStatus::Inactive => "inactive",
        }
    }
}

impl FromStr for UserRole {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            _ => Ok(UserRole::User),
        }
    }
}

impl FromStr for UserStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(UserStatus::Active),
            "inactive" => Ok(UserStatus::Inactive),
            _ => Ok(UserStatus::Active),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
