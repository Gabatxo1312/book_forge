use std::{collections::HashMap, sync::Arc};
use askama::Template;

use axum::{extract::State, http::StatusCode, response::Html, routing::get, Router};

use crate::{config::AppConfig};
use sea_orm::{ EntityTrait };
use entity::book::{Entity as Book, Model};

pub mod users_handler;

pub fn create_router(config: Arc<AppConfig>) -> Router {
    Router::new()
        .route("/", get(root))
        .merge(users_handler::routes())
        .with_state(config)
}

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate {
    books: Vec<Model>
}

// basic handler that responds with a static string
async fn root(
    State(state): State<Arc<AppConfig>>
) -> Result<Html<String>, StatusCode> {
    let all_books = Book::find().all(&state.db).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(
            RootTemplate {
                books: all_books,
            }.render().unwrap()
    ))
}
