use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub refresh_token_expired: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshTokenResponseDto {
    pub status: String,
    pub data: TokenData,
}
