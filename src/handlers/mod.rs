use std::sync::Arc;
use askama::Template;

use axum::{extract::State, response::Html, routing::get, Router};

use crate::{config::AppConfig, models::{Book, User}, repositories::{book_repository::BookRepository, DatabaseConnection}};

pub mod books_handler;
pub mod users_handler;

pub fn create_router(config: Arc<AppConfig>) -> Router {
    Router::new()
        .route("/", get(root))
        .merge(books_handler::routes())
        .merge(users_handler::routes())
        .with_state(config)
}

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate {
    books: Vec<(Book, User, Option<User>)>
}

// basic handler that responds with a static string
async fn root(
    State(state): State<Arc<AppConfig>>,
) -> Html<String> {
    let mut db = DatabaseConnection::new(&state.database_url)
        .expect("Failed to create Connection.");
    
    let all_books = BookRepository::get_all_books_with_users(&mut db)
        .expect("Failed to load users");

    Html(
        RootTemplate {
            books: all_books 
        }.render().unwrap()
    )
}
