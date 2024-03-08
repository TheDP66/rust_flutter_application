use redis::Client;
use sqlx::MySqlPool;
use utils::config::Config;

pub mod dtos;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod schemas;
pub mod services;
pub mod utils;

pub struct AppState {
    pub db: MySqlPool,
    pub config: Config,
    pub redis_client: Client,
}
