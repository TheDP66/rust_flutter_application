use actix_web::web::Json;
use sqlx::{mysql::MySqlQueryResult, MySqlPool};

use crate::{repositories::auth_repository, schemas::auth::RegisterUserSchema};

#[derive(Debug)]
pub struct AuthService {
    pool: MySqlPool,
}

impl AuthService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(
        &self,
        user_id: &String,
        body: Json<RegisterUserSchema>,
    ) -> Result<MySqlQueryResult, String> {
        let query_result = auth_repository::register_user(&user_id, &body, self.pool.clone()).await;

        Ok(query_result?)
    }
}
