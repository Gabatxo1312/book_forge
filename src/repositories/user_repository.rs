use diesel::{query_dsl::methods::SelectDsl, QueryResult, RunQueryDsl, SelectableHelper};

use crate::{models::{NewUser, User}, schema::users};

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
}
