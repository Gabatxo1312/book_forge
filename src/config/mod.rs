use std::{env, sync::Arc};

use dotenvy::dotenv;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Arc<AppConfig> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");

        Arc::new(Self {
            database_url: database_url.clone()
        })
    }
}

