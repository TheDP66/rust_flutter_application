use utoipa::ToSchema;

#[derive(Debug, Clone, ToSchema)]
pub struct UpdatePhotoUserSchema {
    pub file: Option<Vec<u8>>,
}
