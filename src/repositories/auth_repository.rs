use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::{models::user::UserModel, schemas::auth::RegisterUserSchema};

pub async fn insert_user(
    pool: MySqlPool,
    body: &Json<RegisterUserSchema>,
) -> Result<UserModel, String> {
    let query_result = query_as!(r#""#);

    Ok(user)
}
