use std::{env, sync::Arc};

use dotenvy::dotenv;

#[derive(Clone)]
pub struct AppConfig {
    pub db: sea_orm::DatabaseConnection,
}

impl AppConfig {
    pub async fn from_env() -> Result<Arc<AppConfig>, sea_orm::DbErr> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("Error to retrieve DATABASE_URL");

        let connection = sea_orm::Database::connect(&database_url).await?;

        Ok(Arc::new(Self {
            db: connection.clone()
        }))
    }
}
