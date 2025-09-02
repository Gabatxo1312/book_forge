use std::{collections::HashMap, sync::Arc};

use askama::Template;
use axum::{body::Body, extract::{Path, Query, State}, http::{Response, StatusCode}, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router};

use crate::{config::AppConfig, models::{Book, NewBook, User}, repositories::{book_repository::BookRepository, user_repository::UserRepository, DatabaseConnection}};

use crate::services::api::open_library;
use crate::services::api::errors;

pub fn routes() -> Router<Arc<AppConfig>> {
    Router::new()
        .route("/books", get(get_all_books))
        .route("/books", post(create_book))
        .route("/books/new", get(new_book))
        .route("/books/:id/edit", get(edit_book))
        .route("/books/:id", post(update_book))
        .route("/books/search", get(search_book_in_open_library))
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

#[derive(Template)]
#[template(path = "books/new.html")]
struct NewBookTemplate {
    users: Vec<User>
}

async fn new_book(
    State(state): State<Arc<AppConfig>>
) -> Html<String> {
    let mut db = DatabaseConnection::new(&state.database_url)
        .expect("Failed to create Connection.");
    
    let users = UserRepository::get_all_users(&mut db)
        .expect("Failed to load users");

    Html(
        NewBookTemplate {
            users: users
        }.render().unwrap()
    )
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

#[derive(Template)]
#[template(path = "books/edit.html")]
struct EditBookTemplate {
    users: Vec<User>,
    book: Book
}

async fn edit_book(
    State(state): State<Arc<AppConfig>>,
    Path(id): Path<i32>
) -> Html<String> {
    let mut db = DatabaseConnection::new(&state.database_url)
        .expect("Failed to create Connection.");
    
    let users = UserRepository::get_all_users(&mut db)
        .expect("Failed to load users");
    
    let book = BookRepository::get_book_by_id(&mut db, id)
        .expect("Failed to load user");

    Html(
        EditBookTemplate {
            users: users,
            book: book
        }.render().unwrap()
    )
}

async fn update_book(
    State(state): State<Arc<AppConfig>>,
    Path(id): Path<i32>,
    Form(payload): Form<NewBook>
) -> Response<Body> {
    let mut db = DatabaseConnection::new(&state.database_url)
        .expect("Failed to create Connection.");
    
    match BookRepository::update_book(&mut db, id, &payload) {
        Ok(_) => Redirect::to("/").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

async fn search_book_in_open_library(
    Query(params): Query<HashMap<String, String>>,
    State(_state): State<Arc<AppConfig>>
) -> impl IntoResponse {
    let params_name = String::from("query");
    let query_value = params.get(&params_name);

    match query_value {
        Some(val) => {
            match open_library::get_book_from_api(Some(val)).await {
                Ok(books) => Ok(Json(books)),
                Err(e) => Err(errors::ApiError::OpenLibraryError(e.to_string()).into_response())
            }
        },
        None => Err(errors::ApiError::QueryParamMissing.into_response())
    }
}

