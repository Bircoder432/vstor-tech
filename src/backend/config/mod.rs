pub mod database;

use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub access_token: Option<String>,
    pub port: u16,
    pub database: database::DatabaseConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let access_token = env::var("ACCESS_TOKEN").ok();

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidPort)?;

        let db_type = match env::var("DB_TYPE")
            .unwrap_or_else(|_| "inmemory".to_string())
            .as_str()
        {
            "inmemory" => database::DatabaseType::InMemory,
            "spacetime" => database::DatabaseType::Spacetime,
            "postgres" => database::DatabaseType::Postgres,
            _ => database::DatabaseType::InMemory,
        };

        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "memory".to_string());

        Ok(Config {
            access_token,
            port,
            database: database::DatabaseConfig {
                db_type,
                url: db_url,
            },
        })
    }

    pub fn server_address(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Invalid port number")]
    InvalidPort,
    #[error("Environment variable error: {0}")]
    Env(#[from] env::VarError),
}
