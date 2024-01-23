use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Moderator,
    User,
}

impl UserRole {
    pub fn to_str(&self) -> &str {
        match self {
            UserRole::Admin => "admin",
            UserRole::Moderator => "moderator",
            UserRole::User => "user",
        }
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => UserRole::Admin,
            "moderator" => UserRole::Moderator,
            "user" => UserRole::User,
            _ => UserRole::User, // Default to UserRole::User or handle as needed
        }
    }
}

#[derive(Debug, Deserialize, sqlx::FromRow, sqlx::Type, Serialize, Clone)]
pub struct UserModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub photo: String,
    pub verified: i8,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Into<UserDto> for UserModel {
    fn into(self) -> UserDto {
        UserDto {
            id: self.id,
            name: self.name,
            email: self.email,
            role: self.role,
            photo: self.photo,
            verified: self.verified != 0,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
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
