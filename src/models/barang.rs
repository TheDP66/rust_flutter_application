use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::dtos::barang::BarangDto;

#[derive(Debug, Deserialize, sqlx::FromRow, sqlx::Type, Serialize, Clone)]
pub struct BarangModel {
    pub id: String,
    pub name: String,
    pub price: i32,
    pub stock: i32,
    // #[serde(rename = "expiredAt")]
    pub expired_at: Option<NaiveDate>,
    // #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    // #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Into<BarangDto> for BarangModel {
    fn into(self) -> BarangDto {
        BarangDto {
            id: self.id,
            name: self.name,
            price: self.price,
            stock: self.stock,
            expired_at: self.expired_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
