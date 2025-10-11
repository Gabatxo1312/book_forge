use std::sync::Arc;

use askama::Template;
use sea_orm::{ ActiveModelTrait, DeleteResult, EntityTrait, Set };
use axum::{extract::{Path, State}, http::StatusCode, response::{Html, Redirect}, routing::{get, post}, Form, Router};

use crate::config::AppState;

use entity::user;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(get_all_users))
        .route("/users", post(create_user))
        .route("/users/:id/delete", post(delete_user))
        .route("/users/:id/edit", get(edit_user))
        .route("/users/:id", post(update_user))
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

#[derive(Template)]
#[template(path = "users/edit.html")]
struct EditUserTemplate {
    user: user::Model,
}

async fn edit_user(
    State(state): State<Arc<AppConfig>>,
    Path(id): Path<i32>
) -> Result<Html<String>, StatusCode> {
    let user: Option<user::Model> = user::Entity::find_by_id(id).one(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    match user {
        Some(user) => {
            Ok(Html(
                    EditUserTemplate {
                        user: user
                    }.render().unwrap()
                )
            )
        },
        None => Err(StatusCode::NOT_FOUND)
    }
}

async fn update_user(
    State(state): State<Arc<AppConfig>>,
    Path(id): Path<i32>,
    Form(payload): Form<user::Model>
) -> Result<Redirect, StatusCode> {
    let user_by_id = user::Entity::find_by_id(id).one(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let mut user_by_id: user::ActiveModel = user_by_id.unwrap().into();

    user_by_id.name = Set(payload.name);

    let _: user::Model = user_by_id.update(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Redirect::to("/users"))
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

async fn delete_user(
    State(state): State<Arc<AppConfig>>,
    Path(id): Path<i32>
) -> Result<Redirect, StatusCode> {
    let _: DeleteResult = user::Entity::delete_by_id(id)
        .exec(&state.db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Redirect::to("/users"))
}
