use actix_web::web::Json;
use sqlx::{mysql::MySqlQueryResult, MySqlPool};

use crate::{
    models::barang::BarangModel, repositories::barang_repository,
    schemas::barang::InsertBarangSchema,
};

#[derive(Debug)]
pub struct BarangService {
    pool: MySqlPool,
}

impl BarangService {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn insert_barang(
        &self,
        barang_id: &String,
        body: Json<InsertBarangSchema>,
    ) -> Result<MySqlQueryResult, String> {
        let query_result =
            barang_repository::insert_barang(&barang_id, &body, self.pool.clone()).await;

        Ok(query_result?)
    }

    pub async fn get_barang_by_id(&self, barang_id: &str) -> Result<BarangModel, sqlx::Error> {
        let barang = barang_repository::get_barang_by_id(barang_id, self.pool.clone()).await?;

        Ok(barang)
    }

    pub async fn get_barang_by_name(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<BarangModel>, sqlx::Error> {
        let barang = barang_repository::get_barang_by_name(name, self.pool.clone()).await?;

        Ok(barang)
    }
}
