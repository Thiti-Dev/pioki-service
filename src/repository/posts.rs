use std::rc::Rc;
use diesel::prelude::*;

use diesel::associations::HasTable;
use diesel::SelectableHelper;

use crate::domains::inputs::posts::PostLookupWhereClause;
use crate::models::{Post, User};

pub struct PostRepository{
    pub db_pool: Rc<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>>>,
}

impl PostRepository{
    pub fn find_all(&self, clause:  Option<PostLookupWhereClause>) -> Result<Vec<(Post, User)>, diesel::result::Error>{
        use crate::schema::users::dsl::{users,pioki_id};
        use crate::schema::posts::dsl::*;
        let connection = &mut self.db_pool.get().unwrap();

        if clause.is_some(){
            let clause_data = clause.unwrap();

            let mut query = posts::table().into_boxed();

            if let Some(user_id) = clause_data.user_id {
                query = query.filter(creator_id.eq(user_id));
            }

            return query
            .inner_join(users.on(pioki_id.eq(creator_id)))
            .select((Post::as_select(), User::as_select()))
            .load::<(Post, User)>(connection)
            
        }else{
            return posts
            .inner_join(users.on(pioki_id.eq(creator_id)))
            .select((Post::as_select(), User::as_select()))
            .load::<(Post, User)>(connection)
        }

    }
}