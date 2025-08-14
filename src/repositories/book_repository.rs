use diesel::{query_dsl::methods::SelectDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::{models::{Book, NewBook}, schema::books};

use super::DatabaseConnection;

pub struct BookRepository;

impl BookRepository {
    pub fn get_all_books(db: &mut DatabaseConnection) -> QueryResult<Vec<Book>> {
        books::table
            .select(Book::as_select())
            .load(db.connection())
    }

    pub fn create_book(db: &mut DatabaseConnection, new_book: &NewBook) -> QueryResult<Book> {
        diesel::insert_into(books::table)
            .values(new_book)
            .get_result(db.connection())
    }
}
