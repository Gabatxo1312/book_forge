use std::sync::Arc;

use axum::{body::Body, extract::State, http::{Response, StatusCode}, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router};

use crate::{config::AppConfig, handlers::helpers::render_template, models::{Book, NewBook}, repository::Repository};

pub fn routes() -> Router<Arc<AppConfig>> {
    Router::new()
        .route("/books", get(get_all_books))
        .route("/books/new", get(new_book))
        .route("/books", post(create_book))
}

async fn get_all_books(
    State(state): State<Arc<AppConfig>>
) -> Json<Vec<Book>> {
    let mut repo = Repository::new(&state.database_url)
        .expect("Failed to create repository");

    let books = repo.get_all_books()
        .expect("Failed to get books");

    Json(books)
}

async fn new_book(
    State(state): State<Arc<AppConfig>>
) -> Html<String> {
    let mut repo = Repository::new(&state.database_url)
        .expect("Failed to create repository");

    let users = repo.get_all_users().expect("Failed to load users");

    let context = minijinja::context! {
        users => users
    };

    render_template(axum::extract::State(state), "books/new.html", context)
}

async fn create_book(
    State(state): State<Arc<AppConfig>>,
    Form(payload): Form<NewBook>
) -> Response<Body> {
    let mut repo = Repository::new(&state.database_url)
        .expect("Failed to create repository");

    match repo.create_book(&payload) {
        Ok(_) => Redirect::to("/").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
