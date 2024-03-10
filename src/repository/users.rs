use diesel::SelectableHelper;
use diesel::prelude::*;


use crate::db_connection::DbPool;
use crate::models::User;


pub struct UserRepository{
    pub db_pool: DbPool,
}

impl UserRepository{
    pub fn get_users(&self) -> Vec<User>{
        use crate::schema::users::dsl::*;
        let connection = &mut self.db_pool.get().unwrap();

        let results = users
            // .limit(5)
            .select(User::as_select())
            .load(connection)
            .expect("Error loading posts");

        results
    }
}