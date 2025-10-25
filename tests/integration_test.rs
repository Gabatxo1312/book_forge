mod common;
use common::setup_test::TestSetup;

#[tokio::test]
async fn test_root() {
    let setup = TestSetup::setup_test_db(false).await;
    let server = setup.server;

    let response = server.get("/").await;
    response.assert_status_ok();
}

#[tokio::test]
async fn test_users() {
    let setup = TestSetup::setup_test_db(true).await;
    let server = setup.server;

    let response = server.get("/users").await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_show_book() {
    let setup = TestSetup::setup_test_db(true).await;
    let server = setup.server;

    let response = server.get("/books/1").await;

    response.assert_status_ok();
}
