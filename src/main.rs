use book_forge::{ config::AppConfig, handlers::create_router };

#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    let state = AppConfig::from_env().await?;

    let app = create_router(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
