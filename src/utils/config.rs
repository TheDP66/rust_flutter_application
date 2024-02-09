fn get_env_var(var_name: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| panic!("{} must be set", var_name))
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
    pub storage_dir: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url = get_env_var("DATABASE_URL");
        let jwt_secret = get_env_var("JWT_SECRET_KEY");
        let jwt_mexage = get_env_var("JWT_MAXAGE");
        let port = get_env_var("PORT");
        let storage_dir = get_env_var("STORAGE_DIR");

        Config {
            database_url,
            storage_dir,
            jwt_secret,
            jwt_maxage: jwt_mexage.parse::<i64>().unwrap(),
            port: port.parse::<u16>().unwrap(),
        }
    }
}
