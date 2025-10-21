use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id).not_null())
                    .col(string(User::Name).not_null())
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Book::Table)
                    .if_not_exists()
                    .col(pk_auto(Book::Id).not_null())
                    .col(string(Book::Title).not_null())
                    .col(string(Book::Description).null())
                    .col(string(Book::OpenLibraryLink).null())
                    .col(string(Book::CoverUrl).null())
                    .col(integer(Book::OwnerId).not_null())
                    .col(ColumnDef::new(Book::CurrentHolderId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book-owner_id")
                            .from(Book::Table, Book::OwnerId)
                            .to(User::Table, User::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-book-current_holder_id")
                            .from(Book::Table, Book::CurrentHolderId)
                            .to(User::Table, User::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .drop_table(Table::drop().table(Book::Table).to_owned())
            .await;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Book {
    Table,
    Id,
    Title,
    OwnerId,
    CurrentHolderId,
    Description,
    OpenLibraryLink,
    CoverUrl
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name
}
