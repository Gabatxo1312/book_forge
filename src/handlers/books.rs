use std::sync::Arc;

use axum::{body::Body, extract::State, http::{Response, StatusCode}, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router};

use crate::{config::AppConfig, handlers::helpers::render_template, models::{Book, NewBook}, repositories::{book_repository::BookRepository, user_repository::UserRepository, DatabaseConnection}};

pub fn routes() -> Router<Arc<AppConfig>> {
    Router::new()
        .route("/books", get(get_all_books))
        .route("/books/new", get(new_book))
        .route("/books", post(create_book))
}

async fn get_all_books(
    State(state): State<Arc<AppConfig>>
) -> Json<Vec<Book>> {
    let mut db = DatabaseConnection::new(&state.database_url)
        .expect("Failed to create Connection.");
    
    let books = BookRepository::get_all_books(&mut db)
        .expect("Failed to load users");

    Json(books)
}

async fn new_book(
    State(state): State<Arc<AppConfig>>
) -> Html<String> {
    let mut db = DatabaseConnection::new(&state.database_url)
        .expect("Failed to create Connection.");
    
    let users = UserRepository::get_all_users(&mut db)
        .expect("Failed to load users");

    let context = minijinja::context! {
        users => users
    };

    render_template(axum::extract::State(state), "books/new.html", context)
}

async fn create_book(
    State(state): State<Arc<AppConfig>>,
    Form(payload): Form<NewBook>
) -> Response<Body> {
    let mut db = DatabaseConnection::new(&state.database_url)
        .expect("Failed to create Connection.");
    
    match BookRepository::create_book(&mut db, &payload) {
        Ok(_) => Redirect::to("/").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
