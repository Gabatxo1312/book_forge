use diesel::{Connection, ConnectionError, SqliteConnection};

pub mod user_repository;
pub mod book_repository;

pub struct DatabaseConnection {
    pub conn: SqliteConnection
}

impl DatabaseConnection {
    pub fn new(database_url: &str) -> Result<Self, ConnectionError> {
        let conn = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Ok(Self { conn })
    }

    pub fn connection(&mut self) -> &mut SqliteConnection {
        &mut self.conn
    }
}
