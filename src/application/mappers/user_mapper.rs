use std::str::FromStr;

use crate::domain::{
    dtos::users::user_row_dto::UserRowDto,
    entities::user_entity::{UserEntity, UserRole, UserStatus},
};

pub struct UserMapper;

impl UserMapper {
    pub fn row_to_entity(row: UserRowDto) -> UserEntity {
        UserEntity {
            id: row.id,
            email: row.email,
            password: row.password,
            name: row.name,
            role: UserRole::from_str(&row.role).unwrap_or(UserRole::User),
            status: UserStatus::from_str(&row.status).unwrap_or(UserStatus::Active),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }

    pub fn entity_to_row(entity: UserEntity) -> UserRowDto {
        UserRowDto {
            id: entity.id,
            email: entity.email,
            password: entity.password,
            name: entity.name,
            role: entity.role.as_str().to_string(),
            status: entity.status.as_str().to_string(),
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}
