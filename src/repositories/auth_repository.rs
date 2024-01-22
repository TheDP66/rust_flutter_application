use actix_web::web::Json;
use sqlx::MySqlPool;

use crate::{models::user::UserModel, schemas::auth::RegisterUserSchema};

pub async fn insert_user(
    pool: MySqlPool,
    body: Json<RegisterUserSchema>,
) -> Result<UserModel, String> {
    let note = sqlx::query_as!(
        UserModel,
        r#"INSERT INTO users (name, email, password) VALUES ($1, $2, $3) 
        RETURNING id, name, email, password, photo, verified, created_at, updated_at, role as "role: UserRole""#,
        note_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(note)
}
