use diesel::{query_dsl::methods::SelectDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::{models::{NewUser, User}, schema::users};
use diesel::query_dsl::methods::FindDsl;

use super::DatabaseConnection;

pub struct UserRepository;

impl UserRepository {
    pub fn get_all_users(db: &mut DatabaseConnection) -> QueryResult<Vec<User>> {
        users::table
            .select(User::as_select())
            .load(db.connection())
    }

    pub fn create_user(db: &mut DatabaseConnection, new_user: &NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(db.connection())
    }

    pub fn get_user_by_id(db: &mut DatabaseConnection, user_id: i32) -> QueryResult<User> {
        users::dsl::users.find(user_id)
            .select(User::as_select())
            .first(db.connection())
    }
}
