use std::sync::Arc;

use askama::Template;
use sea_orm::{ EntityTrait, Set, ActiveModelTrait };
use axum::{extract::State, http::{StatusCode}, response::{Html, Redirect}, routing::{get, post}, Form, Router};

use crate::config::AppState;

use entity::user;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(get_all_users))
        .route("/users", post(create_user))
}

#[derive(Template)]
#[template(path = "users/index.html")]
struct UsersIndexTemplate {
    users: Vec<user::Model>,
}

async fn get_all_users(
    State(state): State<Arc<AppState>>
) -> Result<Html<String>, StatusCode> {
    let users = user::Entity::find().all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(
        UsersIndexTemplate {
            users: users
        }.render().unwrap()
    ))
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Form(payload): Form<user::Model>
) -> Result<Redirect, StatusCode> {
    let new_user = user::ActiveModel {
        name: Set(payload.name.to_owned()),
        ..Default::default()
    };

    new_user.insert(&state.db).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/users"))
}
