use diesel::associations::HasTable;
use diesel::SelectableHelper;
use diesel::prelude::*;


use crate::db_connection::DbPool;
use crate::models::NewUser;
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
    pub fn create_user(&self, pioki_id: &str,display_name: &str,profile_picture_url: Option<&str>) -> Result<User, diesel::result::Error>{
        use crate::schema::users::dsl::users;
        let connection = &mut self.db_pool.get().unwrap();
        let new_user = NewUser { pioki_id,oauth_display_name: display_name,is_active: true,oauth_profile_picture:  profile_picture_url};

        diesel::insert_into(users::table())
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(connection)
    }
}