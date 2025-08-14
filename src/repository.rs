use diesel::{
    Connection,
    ConnectionError,
    QueryResult,
    SqliteConnection,
    SelectableHelper,
    prelude::*
};
use crate::{models::{Author, Book, NewAuthor, NewBook, NewBookAuthor, NewUser, User}, schema::{authors, books, users}};

pub struct Repository {
    conn: SqliteConnection
}

impl Repository {
    pub fn new(database_url: &str) -> Result<Self, ConnectionError> {
        let conn = SqliteConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
        Ok(Repository { conn })
    }

    pub fn get_all_users(&mut self) -> QueryResult<Vec<User>> {
        users::table
            .select(User::as_select())
            .load(&mut self.conn)
    }

    pub fn create_user(&mut self, new_user: &NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(&mut self.conn)
    }

    pub fn get_all_books(&mut self) -> QueryResult<Vec<Book>> {
        books::table
            .select(Book::as_select())
            .load(&mut self.conn)
    }

    pub fn create_book(&mut self, new_book: &NewBook) -> QueryResult<Book> {
        diesel::insert_into(books::table)
            .values(new_book)
            .get_result(&mut self.conn)
    }

    pub fn get_all_author(&mut self) -> QueryResult<Vec<Author>> {
        authors::table
            .select(Author::as_select())
            .load(&mut self.conn)
    }

    pub fn create_author(&mut self, new_author: &NewAuthor) -> QueryResult<Vec<Author>> {
        diesel::insert_into(authors::table)
            .values(new_author)
            .load(&mut self.conn)
    }
}

pub fn get_authors_for_book(conn: &mut SqliteConnection, book_id_param: i32) -> QueryResult<Vec<Author>> {
    use crate::schema::authors;
    use crate::schema::books_authors;
    use crate::schema::books_authors::dsl::*;

    authors::table
        .inner_join(books_authors::table)
        .filter(book_id.eq(book_id_param))
        .select(Author::as_select())
        .load(conn)
}

pub fn add_author_to_book(
    conn: &mut SqliteConnection,
    book_id: i32,
    author_id: i32,
) -> QueryResult<()> {
    use crate::schema::books_authors;

    let new_relation = NewBookAuthor {
        book_id,
        author_id,
    };

    diesel::insert_into(books_authors::table)
        .values(&new_relation)
        .execute(conn)?;

    Ok(())
}
