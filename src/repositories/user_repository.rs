use sqlx::{mysql::MySqlQueryResult, MySqlPool};

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

pub async fn update_user(
    user_id: &str,
    photo: Option<&str>,
    pool: MySqlPool,
) -> Result<MySqlQueryResult, String> {
    let query_result = sqlx::query(
        r#"
            UPDATE users 
            SET photo = ? 
            WHERE id = ?
        "#,
    )
    .bind(match photo {
        Some(photo_id) => photo_id,
        None => "default.png",
    })
    .bind(user_id)
    .execute(&pool)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    Ok(query_result?)
}
