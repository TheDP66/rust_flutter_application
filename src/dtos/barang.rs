use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::barang::BarangModel;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct BarangDto {
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

impl Into<BarangModel> for BarangDto {
    fn into(self) -> BarangModel {
        BarangModel {
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

impl BarangDto {
    pub fn filter(barang: &BarangModel) -> Self {
        BarangDto {
            id: barang.id.clone(),
            name: barang.name.clone(),
            price: barang.price.clone(),
            stock: barang.stock.clone(),
            expired_at: barang.expired_at,
            created_at: barang.created_at,
            updated_at: barang.updated_at,
        }
    }

    pub fn filter_iter(barangs: &[BarangModel]) -> Vec<BarangDto> {
        barangs.iter().map(BarangDto::filter).collect()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BarangsResponseDto {
    pub status: String,
    pub data: BarangsData,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BarangsData {
    pub barang: Vec<BarangDto>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BarangResponseDto {
    pub status: String,
    pub data: BarangData,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct BarangData {
    pub barang: BarangDto,
}
