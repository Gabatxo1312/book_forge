pub use sea_orm_migration::prelude::*;

mod m20251003_201722_create_book;
mod m20251021_202920_add_authors_to_book;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251003_201722_create_book::Migration),
            Box::new(m20251021_202920_add_authors_to_book::Migration),
        ]
    }
}
