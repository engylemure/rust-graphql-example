use dotenv::dotenv;
use std::env;

pub struct EnvironmentValues {
    pub domain: String,
    pub redis_url: String,
    pub database_url: String,
    pub jwt_private_key: String,
    pub server_port: i16,
    pub rust_env: String,
    pub api_version_date: String,
}

impl EnvironmentValues {
    fn init() -> Self {
        dotenv().ok();
        Self {
            domain: env::var("DOMAIN").unwrap_or("localhost".into()),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            jwt_private_key: env::var("JWT_PRIVATE_KEY").unwrap_or("my secret".into()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| String::from("80"))
                .parse()
                .expect("SERVER_PORT must be a number"),
            rust_env: env::var("RUST_ENV").unwrap_or("dev".into()),
            api_version_date: env::var("API_VERSION_DATE").unwrap_or("2020-03-31".into()),
        }
    }
}

lazy_static! {
    pub static ref ENV: EnvironmentValues = EnvironmentValues::init();
}
