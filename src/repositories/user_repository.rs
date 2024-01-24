use sqlx::MySqlPool;

use crate::models::user::UserModel;

pub async fn get_user(
    user_id: Option<&str>,
    name: Option<&str>,
    email: Option<&str>,
    pool: MySqlPool,
) -> Result<Option<UserModel>, sqlx::Error> {
    let user = sqlx::query_as!(
        UserModel,
        r#"
            SELECT *
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
