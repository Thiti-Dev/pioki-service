// use std::fmt::{Debug, Error};
use std::rc::Rc;
use diesel::dsl::count_star;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use diesel::associations::HasTable;
use diesel::SelectableHelper;

use crate::domains::inputs::posts::PostLookupWhereClause;
use crate::dtos::posts::CreatePostDTO;
use crate::models::{NewPost, NewPostKeeper, Post, PostKeeper, User};

pub enum PostKeepingError{
    AlreadyInteractedError,
    RollbackError,
    NoMoreQuota,
    DatabaseError(String)
}

impl From<diesel::result::Error> for PostKeepingError {
    fn from(err: diesel::result::Error) -> Self {
        // Here you can convert the diesel error into your custom error
        // This is just a simple example, adjust it according to your needs
        PostKeepingError::DatabaseError(err.to_string())
    }
}

#[derive(Clone)]
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

    pub fn create_post(&self, user_id: String, dto: CreatePostDTO) -> Result<Post, diesel::result::Error>{
        use crate::schema::posts::dsl::*;
        let connection = &mut self.db_pool.get().unwrap();


         diesel::insert_into(posts::table())
        .values(&NewPost{
            content: dto.content,
            creator_id: user_id,
            origin_quota_limit: dto.quota_limit as i32,
            quota_left: dto.quota_limit as i32,
            spoiler_header: dto.spoiler_header
        })
        .returning(Post::as_returning())
        .get_result::<Post>(connection)
    }

    fn check_if_post_is_already_kept_by_user(&self, user_id: String, post_id: i32) -> bool{
        use crate::schema::post_keepers::dsl::{post_id as post_id_col,post_keepers,pioki_id};
        let connection = &mut self.db_pool.get().unwrap();

        let count: i64 = post_keepers.select(count_star()).filter(pioki_id.eq(user_id).and(post_id_col.eq(post_id))).first::<i64>(connection).expect("failed getting count by check_if_post_is_already_kept_by_user");
        
        return count != 0
    }

    pub fn keep_post(&self, user_id: String, post_id: i32) -> Result<PostKeeper,PostKeepingError>{
        use crate::schema::post_keepers::dsl::{post_keepers,post_id as post_id_col,pass_along_at};
        use crate::schema::posts::dsl::*;
        let connection = &mut self.db_pool.get().unwrap();

        // check first if this user hasn't already kept/already pass along this post
        let has_already_interected_with_post = self.check_if_post_is_already_kept_by_user(user_id.to_string(),post_id);
        if has_already_interected_with_post{
            return Err(PostKeepingError::AlreadyInteractedError)
        }


        let transaction_result = connection.transaction::<_, PostKeepingError, _>(|conn| {

            let post_res: Result<Post,diesel::result::Error> = posts.select(Post::as_select())
            .find(post_id)
            .for_update()
            .first::<Post>(conn); // selection will be blocked by this

            match post_res {
                Ok(post) => {
                    // create the post_keepers
                    let count_res: Result<i64,diesel::result::Error> = post_keepers.select(count_star()).filter(post_id_col.eq(post_id).and(pass_along_at.is_null())).first::<i64>(conn);
                    if let Ok(keep_count) = count_res{
                        if keep_count as i32 >= post.origin_quota_limit{
                            return Err(PostKeepingError::NoMoreQuota) // could be any error tho, but i am not going to extract later . . . as I will always determine this failure by out-of-quota case
                        }

                        // post is keep-able for now
                        // calling insert here

                        let post_keeper_insert_item = NewPostKeeper{pioki_id: &user_id,post_id};

                        let pk_insertion = diesel::insert_into(post_keepers::table())
                        .values(&post_keeper_insert_item)
                        .returning(PostKeeper::as_returning())
                        .get_result::<PostKeeper>(conn);

                        // lastly decrease the quota left
                        let quota_updation: Result<_, _> = diesel::update(posts.find(post_id)).set(quota_left.eq(post.origin_quota_limit - (keep_count as i32 + 1))).execute(conn); // post.origin_quota_limit - (keep_count as i32 + 1) for re-stamping intregity by the actual counted result
                        if let Err(_) = quota_updation{
                            return Err(PostKeepingError::RollbackError)
                        }

                        if let Ok(pk_res) = pk_insertion{
                            return Ok(pk_res)
                        }else{
                            return Err(PostKeepingError::RollbackError)
                        }
                    }else{
                        // If couldn't count for some reason
                        return Err(PostKeepingError::RollbackError)
                    }
                },
                Err(_) => return Err(PostKeepingError::RollbackError),
            }
        });

        match transaction_result {
            Ok(res) => {
                Ok(res)
            },
            Err(e) => {
                Err(e) // forward the error
            },
        }
    }
}