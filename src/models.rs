use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ErrorResponse {
   pub  error: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub name: String,
}

#[derive(Debug)]
#[derive(Queryable, Selectable, Serialize, AsChangeset)]
#[diesel(table_name = crate::schema::books)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Book{
    pub id: i32,
    pub title: String,
    pub owner_id: i32,
    pub current_holder_id: Option<i32>
}

#[derive(Insertable, Deserialize, Serialize, Debug, AsChangeset)]
#[diesel(table_name = crate::schema::books)]
pub struct NewBook {
    pub title: String,
    pub owner_id: i32,
    pub current_holder_id: Option<i32>
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::authors)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Author {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::authors)]
pub struct NewAuthor {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Queryable, Associations)]
#[diesel(belongs_to(Book))]
#[diesel(belongs_to(Author))]
#[diesel(table_name = crate::schema::books_authors)]
pub struct BookAuthor {
    pub book_id: i32,
    pub author_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::books_authors)]
pub struct NewBookAuthor {
    pub book_id: i32,
    pub author_id: i32,
}
