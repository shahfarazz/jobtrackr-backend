use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok(); // Load .env into environment variables

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .expect("PORT must be a number"),
        }
    }
}
