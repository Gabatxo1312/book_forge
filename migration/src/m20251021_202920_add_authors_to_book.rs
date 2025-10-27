use sea_orm_migration::prelude::*;

use crate::m20251003_201722_create_book::Book;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Book::Table)
                    .add_column(
                        ColumnDef::new(Alias::new("authors"))
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(Table::alter().drop_column("authors").to_owned())
            .await
    }
}
