use sqlx::MySqlPool;

use crate::models::user::UserModel;

pub async fn get_user_by_id(user_id: &String, pool: MySqlPool) -> Result<UserModel, sqlx::Error> {
    let user = sqlx::query_as!(
        UserModel,
        r#"
            SELECT * 
            FROM users 
            WHERE id = ?
        "#,
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(user)
}

pub async fn get_user(
    user_id: Option<&str>,
    name: Option<&str>,
    email: Option<&str>,
    pool: MySqlPool,
) -> Result<Option<UserModel>, sqlx::Error> {
    let user = sqlx::query_as!(
        UserModel,
        r#"
            SELECT id, name, email, password, photo, verified, created_at, updated_at, role
            FROM users 
            WHERE id = ? OR name = ? OR email = ?
        "#,
        user_id,
        name,
        email,
    )
    .fetch_optional(&pool)
    .await?;

    Ok(user)
}
