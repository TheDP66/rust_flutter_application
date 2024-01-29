use sqlx::{mysql::MySqlQueryResult, MySqlPool};

use crate::{models::barang::BarangModel, schemas::barang::InsertBarangSchema};

pub async fn insert_barang(
    barang_id: &String,
    body: &InsertBarangSchema,
    pool: MySqlPool,
) -> Result<MySqlQueryResult, String> {
    let query_result = sqlx::query(
        r#"
            INSERT INTO barang (id, name, price, stock, expired_at)
            VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(barang_id.clone())
    .bind(body.name.to_string())
    .bind(body.price)
    .bind(body.stock)
    .bind(body.expired_at)
    .execute(&pool)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    Ok(query_result?)
}

pub async fn get_barang_by_name(
    name: Option<&str>,
    pool: MySqlPool,
) -> Result<Vec<BarangModel>, sqlx::Error> {
    let names = match name {
        None => "",
        Some(e) => e,
    };

    let name_pattern = format!("%{}%", names);

    let barang = sqlx::query_as!(
        BarangModel,
        r#"
            SELECT *
            FROM barang
            WHERE name LIKE ?
        "#,
        name_pattern,
    )
    .fetch_all(&pool)
    .await?;

    Ok(barang)
}

pub async fn get_barang_by_id(
    barang_id: &str,
    pool: MySqlPool,
) -> Result<BarangModel, sqlx::Error> {
    let barang = sqlx::query_as!(
        BarangModel,
        r#"
            SELECT *
            FROM barang
            WHERE id = ?
            LIMIT 1
        "#,
        barang_id,
    )
    .fetch_one(&pool)
    .await?;

    Ok(barang)
}
