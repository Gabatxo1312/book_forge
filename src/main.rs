use book_forge::{config::AppConfig, handlers::create_router};

#[tokio::main]
async fn main() {
    let state = AppConfig::from_env();

    let app = create_router(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
