use sqlx::MySqlPool;

pub mod handlers;
pub mod models;
pub mod repositories;
pub mod schemas;
pub mod services;
pub mod utils;

pub struct AppState {
    pub db: MySqlPool,
}
