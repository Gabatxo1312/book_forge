mod common;
use common::setup_test::TestSetup;
use entity::book;
use sea_orm::EntityTrait;

// SHOW BOOKS

#[tokio::test]
async fn test_show_book_when_book_doesnt_exist() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let response = server.get("/books/9999").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn test_show_book() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let book = setup.test_data.books[0].clone();

    let response = server.get(&format!("/books/{}", book.id)).await;

    response.assert_status_ok();
}

// NEW BOOK

#[tokio::test]
async fn test_new_book() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let response = server.get("/books/new").await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_post_new_book() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let owner = &setup.test_data.users[0];

    let form_data = [
        ("title", "1984"),
        ("authors", "George Orwell"),
        ("owner_id", &owner.id.to_string()),
        ("description", "Un roman dystopique"),
        ("open_library_link", ""),
        ("cover_url", ""),
        ("current_holder_id", ""),
    ];

    let response = server
        .post("/books")
        .form(&form_data)
        .await;

    response.assert_status_see_other();
    response.assert_header("location", "/");

    let books = book::Entity::find()
        .all(&setup.db)
        .await
        .unwrap();

    // 1 in seed and another created
    assert_eq!(books.len(), 2);

    let new_book = books.iter().find(|b| b.title == "1984").unwrap();

    assert_eq!(new_book.authors, "George Orwell");
    assert_eq!(new_book.owner_id, owner.id);
}

// DELETE BOOK

#[tokio::test]
async fn test_post_delete_book() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let book = setup.test_data.books[0].clone();
    let book_id = book.id;

    let response_before = server.get(&format!("/books/{}", book_id)).await;
    response_before.assert_status_ok();

    let response_delete = server.post(&format!("/books/{}/delete", book_id)).await;
    response_delete.assert_status_see_other();

    let response_after = server.get(&format!("/books/{}", book_id)).await;
    response_after.assert_status_not_found();
}

// EDIT BOOK

#[tokio::test]
async fn test_edit_book() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let book = setup.test_data.books[0].clone();

    let response = server.get(&format!("/books/{}/edit", book.id)).await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_edit_book_when_book_doesnt_exist() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let response = server.get("/books/9999/edit").await;

    response.assert_status_not_found();
}

// UPDATE BOOK

#[tokio::test]
async fn test_post_update_book() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let owner = &setup.test_data.users[0];

    let form_data = [
        ("title", "1984"),
        ("authors", "Karl Marx"),
        ("owner_id", &owner.id.to_string()),
        ("description", "Un roman dystopique"),
        ("open_library_link", ""),
        ("cover_url", ""),
        ("current_holder_id", ""),
    ];

    let response = server
        .post("/books")
        .form(&form_data)
        .await;

    response.assert_status_see_other();

    let books = book::Entity::find()
        .all(&setup.db)
        .await
        .unwrap();

    let new_book = books.iter().find(|b| b.title == "1984").unwrap();

    assert_eq!(new_book.authors, "Karl Marx");
}




