use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::user::{UserModel, UserRole};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub photo: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Into<UserModel> for UserDto {
    fn into(self) -> UserModel {
        UserModel {
            id: self.id,
            name: self.name,
            email: self.email,
            password: "".to_string(),
            role: self.role,
            photo: self.photo,
            verified: if self.verified { 1 } else { 0 },
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserData {
    pub user: UserDto,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub data: TokenData,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenData {
    pub token: String,
}
