// pub fn get_all_author(db: &mut DatabaseConnection) -> QueryResult<Vec<Author>> {
//         authors::table
//             .select(Author::as_select())
//             .load(&mut self.conn)
//     }
//
//     pub fn create_author(db: &mut DatabaseConnection, new_author: &NewAuthor) -> QueryResult<Vec<Author>> {
//         diesel::insert_into(authors::table)
//             .values(new_author)
//             .load(&mut self.conn)
//     }
//
//
// pub fn get_authors_for_book(conn: &mut SqliteConnection, book_id_param: i32) -> QueryResult<Vec<Author>> {
//     use crate::schema::authors;
//     use crate::schema::books_authors;
//     use crate::schema::books_authors::dsl::*;
//
//     authors::table
//         .inner_join(books_authors::table)
//         .filter(book_id.eq(book_id_param))
//         .select(Author::as_select())
//         .load(conn)
// }
//
// pub fn add_author_to_book(
//     conn: &mut SqliteConnection,
//     book_id: i32,
//     author_id: i32,
// ) -> QueryResult<()> {
//     use crate::schema::books_authors;
//
//     let new_relation = NewBookAuthor {
//         book_id,
//         author_id,
//     };
//
//     diesel::insert_into(books_authors::table)
//         .values(&new_relation)
//         .execute(conn)?;
//
//     Ok(())
// }
