use sqlx::MySqlPool;

use crate::{models::user::UserModel, repositories::user_repository};

#[derive(Debug)]
pub struct UserService {
    pool: MySqlPool,
}

impl UserService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn get_user(
        &self,
        user_id: Option<&str>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<UserModel>, sqlx::Error> {
        let user = user_repository::get_user(user_id, name, email, self.pool.clone()).await?;
        Ok(user)
    }
}
