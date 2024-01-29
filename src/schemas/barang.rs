use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct InsertBarangSchema {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(range(min = 0))]
    pub price: i32,
    #[validate(range(min = 0))]
    pub stock: i32,
    pub expired_at: Option<NaiveDate>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetBarangSchema {
    pub name: Option<String>,
}
