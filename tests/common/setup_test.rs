use book_forge::config::AppState;
use std::sync::Arc;

use axum_test::TestServer;

use migration::MigratorTrait;
use sea_orm::{Database, DatabaseConnection};

use crate::common::seed_data::TestData;

#[allow(dead_code)]
pub struct TestSetup {
    pub db: DatabaseConnection,
    pub test_data: TestData,
    pub server: TestServer,
}

impl TestSetup {
    pub async fn setup_test_db() -> Self {
        // Create database in memory
        let db = Database::connect("sqlite::memory:").await.unwrap();

        // Execute migrations
        migration::Migrator::up(&db, None).await.unwrap();

        let test_data = TestData::seed_fake_books_data(&db).await;

        let app_state = Arc::new(AppState {
            db: db.clone(),
            locale: "en".to_string(),
        });

        let app = book_forge::handlers::create_router(app_state);
        let server = TestServer::new(app).unwrap();

        Self {
            db,
            test_data: test_data.unwrap(),
            server,
        }
    }
}
