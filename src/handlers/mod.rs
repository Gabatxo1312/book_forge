use askama::Template;
use std::{collections::HashMap, sync::Arc};

use rust_i18n::t;

use crate::config::AppState;
use crate::helpers::filters;
use axum::extract::Query;
use axum::{Router, extract::State, http::StatusCode, response::Html, routing::get};
use entity::book;
use entity::user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QueryTrait};

pub mod books_handler;
pub mod users_handler;

pub fn create_router(config: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(root))
        .merge(users_handler::routes())
        .merge(books_handler::routes())
        .with_state(config)
}

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate {
    books_with_users: Vec<(book::Model, Option<user::Model>, Option<user::Model>)>,
    users: Vec<user::Model>,
}

// basic handler that responds with a static string
async fn root(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Html<String>, StatusCode> {
    // Get query values from Query extractor
    let query_search: Option<&String> = params.get(&String::from("q"));
    let query_owner: Option<&String> = params.get(&String::from("owner_id"));
    let query_holder: Option<&String> = params.get(&String::from("current_holder_id"));
    let author_search: Option<&String> = params.get(&String::from("authors"));

    // Get all users (possible because there are a few user)
    let users: Vec<user::Model> = user::Entity::find()
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get all books with get filters
    let all_books = book::Entity::find()
        .apply_if(query_owner, |query, value| match value.parse::<i32>() {
            Ok(q) => query.filter(book::Column::OwnerId.eq(q)),
            Err(_) => query,
        })
        .apply_if(query_holder, |query, value| match value.parse::<i32>() {
            Ok(q) => query.filter(book::Column::CurrentHolderId.eq(q)),
            Err(_) => query,
        })
        .apply_if(query_search, |query, value| {
            query.filter(book::Column::Title.contains(String::from(value)))
        })
        .apply_if(author_search, |query, value| {
            query.filter(book::Column::Authors.contains(String::from(value)))
        })
        .order_by_desc(book::Column::Id)
        .all(&state.db)
        .await
        .map_err(|err| {
            println!("{:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let users_hash: HashMap<i32, user::Model> = users
        .clone()
        .into_iter()
        .map(|user| (user.id, user))
        .collect();

    let books_with_users = all_books
        .into_iter()
        .map(|book| {
            let owner = users_hash.get(&book.owner_id).map(|u| (*u).clone());
            let holder = book.current_holder_id.and_then(|current_holder_id| {
                users_hash.get(&current_holder_id).map(|u| (*u).clone())
            });
            (book, owner, holder)
        })
        .collect();

    Ok(Html(
        RootTemplate {
            books_with_users,
            users,
        }
        .render()
        .unwrap(),
    ))
}
