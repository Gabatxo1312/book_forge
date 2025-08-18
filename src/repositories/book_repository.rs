use diesel::{alias, query_dsl::methods::SelectDsl, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper, TextExpressionMethods};

use crate::{models::{Book, NewBook, User}, schema::{books::{self, title}, users}};

use super::DatabaseConnection;

pub struct BookRepository;

impl BookRepository {
    pub fn get_all_books_with_users(db: &mut DatabaseConnection, search_term: Option<String>) -> QueryResult<Vec<(Book, User, Option<User>)>> {
        let holders = alias!(users as holders);
        let base_query = books::table.into_boxed();

        let filtered_query = match search_term {
            Some(term) => base_query.filter(books::title.like(format!("%{}%", term))),
            None => base_query,
        };

        SelectDsl::select(filtered_query 
            .inner_join(users::table.on(books::owner_id.eq(users::id)))
            .left_join(holders.on(books::current_holder_id.eq(holders.field(users::id).nullable()))),
            (
                books::all_columns,
                users::all_columns,
                holders.fields(users::all_columns).nullable()
            ))
            .load::<(Book, User, Option<User>)>(db.connection())
    }

    pub fn get_all_books(db: &mut DatabaseConnection) -> QueryResult<Vec<Book>> {
        SelectDsl::select(books::table, Book::as_select())
            .load(db.connection())
    }

    pub fn get_book_by_id(db: &mut DatabaseConnection, book_id: i32) -> QueryResult<Book> {
        SelectDsl::select(QueryDsl::find(books::dsl::books, book_id), Book::as_select())
            .first(db.connection())
    }

    pub fn create_book(db: &mut DatabaseConnection, new_book: &NewBook) -> QueryResult<Book> {
        diesel::insert_into(books::table)
            .values(new_book)
            .get_result(db.connection())
    }

    pub fn update_book(db: &mut DatabaseConnection, book_id: i32, book: &NewBook) -> QueryResult<Book> {
        diesel::update(QueryDsl::find(books::dsl::books, book_id))
            .set(book)
            .get_result(db.connection())
    }
}
