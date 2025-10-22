use std::{collections::HashMap, sync::Arc};

use askama::Template;
use sea_orm::{ ActiveModelTrait, DeleteResult, EntityTrait, Set };
use axum::{extract::{Path, Query, State}, http::StatusCode, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Form, Json, Router};

use crate::{config::AppState, helpers::convert_params_string_to_id};
use serde::Deserialize;

use rust_i18n::t;

use entity::user;
use entity::book;

use crate::helpers::filters;

use crate::services;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/books/new", get(new_book))
        .route("/books", post(create_book))
        .route("/books/:id/edit", get(edit_book))
        .route("/books/:id", get(show_book).post(update_book))
        .route("/books/:id/delete", post(delete_book))
        .route("/books/search", get(search_book_in_open_library))
} 

#[derive(Deserialize)]
struct BookForm {
    title: String,
    owner_id: i32,
    description: Option<String>,
    open_library_link: Option<String>,
    cover_url: Option<String>,
    current_holder_id: Option<String>,
    authors: String
}

#[derive(Template)]
#[template(path = "books/show.html")]
struct ShowBookTemplate {
    book: book::Model,
    owner: user::Model,
    current_holder: Option<user::Model>
}

async fn show_book(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>
) -> Result<Html<String>, StatusCode> {
    let book = book::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let book = book.unwrap();

    let owner = user::Entity::find_by_id(book.clone().owner_id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let current_holder = match book.clone().current_holder_id {
        Some(b) => user::Entity::find_by_id(b).one(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        None => None
    };

    Ok(Html(
        ShowBookTemplate {
            book: book,
            owner: owner.unwrap(),
            current_holder: current_holder
        }.render().unwrap()
    ))
}

#[derive(Template)]
#[template(path = "books/new.html")]
struct NewBookTemplate {
    users: Vec<user::Model>,
}

async fn new_book(
    State(state): State<Arc<AppState>>
) -> Result<Html<String>, StatusCode> {
    let users = user::Entity::find().all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(
        NewBookTemplate {
            users: users
        }.render().unwrap()
    ))
}

#[derive(Template)]
#[template(path = "books/edit.html")]
struct EditBookTemplate {
    users: Vec<user::Model>,
    book: book::Model
}

async fn edit_book(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>
) -> Result<Html<String>, StatusCode> {
    let users = user::Entity::find().all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let book: Option<book::Model> = book::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match book {
        Some(b) => Ok(Html(EditBookTemplate { book: b, users: users }.render().unwrap())),
        None => Err(StatusCode::NOT_FOUND)
    }
}

async fn update_book(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Form(payload): Form<BookForm>
) -> Result<Redirect, StatusCode> {
    let book_by_id = book::Entity::find_by_id(id).one(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let current_holder_id = convert_params_string_to_id(payload.current_holder_id);

    let mut book_by_id: book::ActiveModel = book_by_id.unwrap().into();

    book_by_id.title = Set(payload.title);
    book_by_id.owner_id = Set(payload.owner_id);
    book_by_id.description= Set(payload.description);
    book_by_id.open_library_link = Set(payload.open_library_link);
    book_by_id.cover_url = Set(payload.cover_url);
    book_by_id.current_holder_id = Set(current_holder_id);
    book_by_id.authors = Set(payload.authors);

    let _: book::Model = book_by_id.update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(format!("/books/{}", id).as_str()))
}

async fn create_book(
    State(state): State<Arc<AppState>>,
    Form(payload): Form<BookForm>
) -> Result<Redirect, StatusCode> {
    let current_holder_id = payload.current_holder_id
        .and_then(|s| if s.is_empty() { None } else { s.parse().ok() });

    let new_book = book::ActiveModel {
        title: Set(payload.title .to_owned()),
        authors: Set(payload.authors),
        owner_id: Set(payload.owner_id.to_owned()),
        current_holder_id: Set(current_holder_id),
        description: Set(payload.description),
        open_library_link: Set(payload.open_library_link),
        cover_url: Set(payload.cover_url),
        ..Default::default()
    };

    new_book.insert(&state.db).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/"))
}

async fn delete_book(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>
) -> Result<Redirect, StatusCode> {
    let _: DeleteResult = book::Entity::delete_by_id(id)
        .exec(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Redirect::to("/"))
}

async fn search_book_in_open_library(
    Query(params): Query<HashMap<String, String>>,
    State(_state): State<Arc<AppState>>
) -> impl IntoResponse {
    let params_name = String::from("query");
    let query_value = params.get(&params_name);

    match query_value {
        Some(val) => {
            match services::open_library::get_book_from_api(Some(val)).await {
                Ok(books) => Ok(Json(books)),
                Err(e) => Err(services::errors::ApiError::OpenLibraryError(e.to_string()).into_response())
            }
        },
        None => Err(services::errors::ApiError::QueryParamMissing.into_response())
    }
}
