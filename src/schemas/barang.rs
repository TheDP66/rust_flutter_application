use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct InsertBarangSchema {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(range(min = 0))]
    pub price: i32,
    #[validate(range(min = 0))]
    pub stock: i32,
    pub expired_at: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, IntoParams)]
pub struct GetBarangSchema {
    pub name: Option<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct SyncBarangSchema {
    pub barang: Vec<InsertBarangSchema>,
}
