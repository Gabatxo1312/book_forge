// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
    }
}

diesel::table! {
    books (id) {
        id -> Integer,
        title -> Text,
        owner_id -> Integer,
        current_holder_id -> Nullable<Integer>,
    }
}

diesel::table! {
    books_authors (book_id, author_id) {
        book_id -> Integer,
        author_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(books_authors -> authors (author_id));
diesel::joinable!(books_authors -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    books,
    books_authors,
    users,
);
