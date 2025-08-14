use std::sync::Arc;

use axum::{body::Body, extract::State, http::{Response, StatusCode}, response::{Html, IntoResponse, Redirect}, routing::{get, post}, Form, Router};

use crate::{config::AppConfig, handlers::helpers::render_template, models::NewUser, repositories::{user_repository::UserRepository, DatabaseConnection}};

pub fn routes() -> Router<Arc<AppConfig>> {
    Router::new()
        .route("/users", get(get_all_users))
        .route("/users", post(create_user))
}

async fn get_all_users(
    State(state): State<Arc<AppConfig>>
) -> Html<String> {
    let mut db = DatabaseConnection::new(&state.database_url)
        .expect("Failed to create Connection.");
    
    let users = UserRepository::get_all_users(&mut db)
        .expect("Failed to get users");

    let context = minijinja::context! {
        users => users
    };

    render_template(axum::extract::State(state), "users/index.html", context)
}

async fn create_user(
    State(state): State<Arc<AppConfig>>,
    Form(payload): Form<NewUser>
) -> Response<Body> {
    let mut db = DatabaseConnection::new(&state.database_url)
        .expect("Failed to create Connection.");
    
    match UserRepository::create_user(&mut db, &payload) {
        Ok(_) => Redirect::to("/users").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
