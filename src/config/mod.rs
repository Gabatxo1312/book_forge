use std::{env, fs, path::Path, sync::Arc};

use serde::Deserialize;

use std::io::Write;

use crate::helpers::errors::AppError;

const DEFAULT_DATABASE_URL: &str = "sqlite://db.sqlite?mode=rwc";

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub config: AppConfig
}

impl AppState {
    pub async fn initialize() -> Result<Arc<AppState>, sea_orm::DbErr> {
        let config = AppConfig::from_cli();

        let _ = Self::create_env_file(&config.database_url);

        let connection = sea_orm::Database::connect(&config.database_url).await?;

        Ok(Arc::new(Self {
            db: connection.clone(),
            config: config
        }))
    }

    fn create_env_file(database_url: &str) -> std::io::Result<()> {
        let env_path = Path::new(".env");

        if !env_path.exists() {
            println!("Creating .env file with DATABASE_URL for sea_orm_cli...");

            let mut file = fs::File::create(env_path)?;
            writeln!(file, "DATABASE_URL={}", database_url)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub locales: String
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_url: DEFAULT_DATABASE_URL.to_string(),
            locales: String::from("en")
        }
    }
}

impl AppConfig {
    pub fn from_cli() -> AppConfig {
        if let Some(path) = env::args().nth(1) {
            Self::from_file(path).unwrap()
        } else {
            AppConfig::default()
        }
    }

    pub fn from_file<T: AsRef<Path>>(path: T) -> Result<AppConfig, AppError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path).map_err(|_| AppError::ConfigError)?;
        let config: AppConfig = toml::from_str(&content)
            .map_err(|_| AppError::ConfigError)?;

        Ok(config)
    }
}
