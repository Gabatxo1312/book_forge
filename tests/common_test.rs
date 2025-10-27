mod common;
use common::setup_test::TestSetup;

// SHOW BOOKS

#[tokio::test]
async fn test_root() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let response = server.get("/").await;
    response.assert_status_ok();
}
