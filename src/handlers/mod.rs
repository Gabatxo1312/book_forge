use std::{collections::HashMap, sync::Arc};
use askama::Template;

use axum::{extract::State, http::StatusCode, response::Html, routing::get, Router};
use crate::{config::AppConfig};
use sea_orm::{ ColumnTrait, EntityTrait, QueryFilter };
use entity::book;
use entity::user;

pub mod users_handler;
pub mod books_handler;

pub fn create_router(config: Arc<AppConfig>) -> Router {
    Router::new()
        .route("/", get(root))
        .merge(users_handler::routes())
        .merge(books_handler::routes())
        .with_state(config)
}

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate {
    books_with_users: Vec<(book::Model, Option<user::Model>, Option<user::Model>)>
}

// basic handler that responds with a static string
async fn root(
    State(state): State<Arc<AppConfig>>
) -> Result<Html<String>, StatusCode> {
    let all_books = book::Entity::find().all(&state.db).await.map_err(|err| {
        println!("{:?}", err);
        StatusCode::INTERNAL_SERVER_ERROR 
    })?;

    let user_ids: Vec<i32> = all_books.iter().flat_map(|book| {
        let mut ids = vec![book.owner_id];
        if let Some(current_holder_id) = book.current_holder_id {
            ids.push(current_holder_id)
        }
        ids
    }).collect();

    // get all users
    let users: Vec<user::Model> = user::Entity::find()
        .filter(user::Column::Id.is_in(user_ids))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users_hash: HashMap<i32, user::Model> = users.into_iter().map(|user| (user.id, user)).collect();

    let books_with_users = all_books.into_iter().map(|book| {
        let owner = users_hash.get(&book.owner_id).map(|u| (*u).clone());
        let holder = book.current_holder_id.and_then(|current_holder_id| users_hash.get(&current_holder_id).map(|u| (*u).clone()));
        (book, owner, holder)
    }).collect();

    Ok(Html(
            RootTemplate {
                books_with_users: books_with_users,
            }.render().unwrap()
    ))
}
