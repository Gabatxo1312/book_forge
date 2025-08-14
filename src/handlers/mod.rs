use std::sync::Arc;

use axum::{extract::State, response::Html, routing::get, Router};

use crate::{config::AppConfig, handlers::helpers::render_template, repository::Repository};

pub mod helpers;
pub mod books;
pub mod users;

pub fn create_router(config: Arc<AppConfig>) -> Router {
    Router::new()
        .route("/", get(root))
        .merge(books::routes())
        .merge(users::routes())
        .with_state(config)
}

// basic handler that responds with a static string
async fn root(
    State(state): State<Arc<AppConfig>>,
) -> Html<String> {
    let mut repo = Repository::new(&state.database_url)
        .expect("Failed to create repository");

    let books = repo.get_all_books()
        .expect("Failed to retrieve books");

    let context = minijinja::context! {
        books => books
    };

    render_template(axum::extract::State(state), "index.html", context)
}
