use sqlx::{mysql::MySqlQueryResult, MySqlPool};

use crate::{schemas::auth::RegisterUserSchema, utils::password};

pub async fn register_user(
    user_id: &String,
    body: &RegisterUserSchema,
    pool: MySqlPool,
) -> Result<MySqlQueryResult, String> {
    let hashed_password = password::hash(&body.password).map_err(|e| (e.to_string()))?;

    let query_result = sqlx::query(
        r#"
            INSERT INTO users (id, name, email, password) 
            VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(user_id.clone())
    .bind(body.name.to_string())
    .bind(body.email.to_string())
    .bind(hashed_password)
    .execute(&pool)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    Ok(query_result?)
}
