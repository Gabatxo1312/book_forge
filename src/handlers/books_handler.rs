use std::{sync::Arc};

use askama::Template;
use sea_orm::{ ActiveModelTrait, DeleteResult, EntityTrait, Set };
use axum::{extract::{Path, State}, http::StatusCode, response::{Html, Redirect}, routing::{get, post}, Form, Router};

use crate::{config::AppConfig, helpers::convert_params_string_to_id};
use serde::Deserialize;

use entity::user;
use entity::book;

pub fn routes() -> Router<Arc<AppConfig>> {
    Router::new()
        .route("/books/new", get(new_book))
        .route("/books", post(create_book))
        .route("/books/:id/edit", get(edit_book))
        .route("/books/:id", get(show_book).post(update_book))
        .route("/books/:id/delete", post(delete_book))
} 

#[derive(Deserialize)]
struct BookForm {
    title: String,
    owner_id: i32,
    current_holder_id: Option<String>
}

#[derive(Template)]
#[template(path = "books/show.html")]
struct ShowBookTemplate {
    book: book::Model,
    owner: user::Model,
    current_holder: Option<user::Model>
}

async fn show_book(
    State(state): State<Arc<AppConfig>>,
    Path(id): Path<i32>
) -> Result<Html<String>, StatusCode> {
    let book = book::Entity::find_by_id(id)
        .one(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let book = book.unwrap();

    // TODO optimize this 2 request into 1
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
    State(state): State<Arc<AppConfig>>
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
    State(state): State<Arc<AppConfig>>,
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
    State(state): State<Arc<AppConfig>>,
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
    book_by_id.current_holder_id = Set(current_holder_id);

    let _: book::Model = book_by_id.update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to(format!("/books/{}", id).as_str()))
}

async fn create_book(
    State(state): State<Arc<AppConfig>>,
    Form(payload): Form<BookForm>
) -> Result<Redirect, StatusCode> {
    let current_holder_id = payload.current_holder_id
        .and_then(|s| if s.is_empty() { None } else { s.parse().ok() });

    let new_book = book::ActiveModel {
        title: Set(payload.title .to_owned()),
        owner_id: Set(payload.owner_id.to_owned()),
        current_holder_id: Set(current_holder_id),
        ..Default::default()
    };

    new_book.insert(&state.db).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/"))
}

async fn delete_book(
    State(state): State<Arc<AppConfig>>,
    Path(id): Path<i32>
) -> Result<Redirect, StatusCode> {
    let _: DeleteResult = book::Entity::delete_by_id(id)
        .exec(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Redirect::to("/"))
}


