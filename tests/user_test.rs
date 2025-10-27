mod common;
use common::setup_test::TestSetup;
use entity::user;
use sea_orm::{EntityTrait, QueryOrder};


// Router::new()
// .route("/users", get(get_all_users))
// .route("/users", post(create_user))
// .route("/users/{id}/delete", post(delete_user))
// .route("/users/{id}/edit", get(edit_user))
// .route("/users/{id}", post(update_user))

// INDEX USERS

#[tokio::test]
async fn test_get_all_users() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let response = server.get("/users").await;

    response.assert_status_ok();
}

// POST NEW USERS

#[tokio::test]
async fn test_create_user() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let form_data = [
        ("name", "Louise Michel new")
    ];

    let response_delete = server.post("/users").form(&form_data).await;

    response_delete.assert_status_see_other();

    let last_user = user::Entity::find()
        .order_by_desc(user::Column::Id)
        .one(&setup.db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(last_user.name, "Louise Michel new")
}

// EDIT USERS

#[tokio::test]
async fn test_edit_user() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let user = setup.test_data.users[0].clone();

    let response = server.get(&format!("/users/{}/edit", user.id)).await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_edit_user_when_id_doesnt_exist() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let response = server.get("/users/9999/edit").await;

    response.assert_status_not_found();
}

#[tokio::test]
async fn test_update_user() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let user = setup.test_data.users[0].clone();

    let form_data = [
        ("name", "Louise Michel")
    ];

    let response = server.post(&format!("/users/{}", user.id)).form(&form_data).await;

    response.assert_status_see_other();

    let user = user::Entity::find_by_id(user.id)
        .one(&setup.db)
        .await
        .unwrap();
    
    assert_eq!(user.unwrap().name, "Louise Michel");
}

// DELETE USERS

#[tokio::test]
async fn test_delete_user() {
    let setup = TestSetup::setup_test_db().await;
    let server = setup.server;

    let user = setup.test_data.users[0].clone();

    let response_delete = server.post(&format!("/books/{}/delete", user.id)).await;
    response_delete.assert_status_see_other();
}

