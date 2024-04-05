// use std::fmt::{Debug, Error};
use std::rc::Rc;
use chrono::NaiveDateTime;
use diesel::dsl::count_star;
use diesel::{prelude::*, sql_query};
use diesel::result::Error as DieselError;
use bigdecimal::{BigDecimal, FromPrimitive};


use diesel::associations::HasTable;
use diesel::sql_types::Numeric;
use diesel::SelectableHelper;
use serde::Serialize;
use tokio::task;

use crate::domains::inputs::posts::PostLookupWhereClause;
use crate::dtos::posts::CreatePostDTO;
use crate::models::{ FeedData, KeepAndPassAlongLog, NewKeepAndPassAlongLog, NewPost, NewPostKeeper, Post, PostKeeper, User};

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

            return query.order(created_at.desc())
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

    pub fn get_post_feeds_of_specific_user(&self, user_id: String) -> Result<Vec<FeedData>, diesel::result::Error>{
        let connection = &mut self.db_pool.get().unwrap();
        let feed_query = sql_query(format!("
                SELECT p.id,p.creator_id, p.spoiler_header, CASE 
                WHEN pk.post_id IS NOT NULL THEN p.content 
                ELSE '#!@#$%-System-Encrypted-#!@#$%' 
                END as content, p.origin_quota_limit,p.quota_left,p.created_at,p.updated_at, u.oauth_display_name, u.oauth_profile_picture
        FROM public.posts p
        LEFT JOIN 
            public.post_keepers pk ON p.id = pk.post_id AND pk.pioki_id = '111610436275740323798'
        LEFT JOIN 
            public.users u ON p.creator_id = u.pioki_id
            
        where creator_id IN (
            SELECT DISTINCT f1.pioki_id
            FROM friends f1
            WHERE pioki_friend_id = '{}'
            AND pioki_id IN (
                SELECT DISTINCT pioki_friend_id
                FROM friends
                WHERE pioki_id = '{}'
                AND pioki_friend_id = f1.pioki_id
            )
        ) order by created_at desc
        "
        ,user_id,user_id))
        .load::<FeedData>(connection);     

        feed_query  
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

    fn _check_if_post_is_already_kept_by_user(&self, user_id: String, post_id: i32) -> bool{
        use crate::schema::keep_and_pass_along_logs::dsl::{keep_and_pass_along_logs,pioki_id,post_id as post_id_col,is_kept};
        let connection = &mut self.db_pool.get().unwrap();

        let count: i64 = keep_and_pass_along_logs.select(count_star()).filter(pioki_id.eq(user_id).and(post_id_col.eq(post_id)).and(is_kept.eq(true))).first::<i64>(connection).expect("failed getting count by check_if_post_is_already_kept_by_user");
        
        return count != 0
    }

    fn check_if_user_ever_keep_this_post(&self, user_id: String, post_id: i32) -> bool{
        use crate::schema::keep_and_pass_along_logs::dsl::{keep_and_pass_along_logs,pioki_id,post_id as post_id_col,is_kept};
        let connection = &mut self.db_pool.get().unwrap();

        let count: i64 = keep_and_pass_along_logs.select(count_star()).filter(pioki_id.eq(user_id).and(post_id_col.eq(post_id))).first::<i64>(connection).expect("failed getting count by check_if_post_is_already_kept_by_user");
        
        return count != 0
    }

    pub fn is_owned(&self, user_id: String, post_id: i32) -> Result<Option<PostKeeper>, diesel::result::Error>{
        use crate::schema::post_keepers::dsl::{post_keepers,post_id as post_id_col,pass_along_at,pioki_id};
        let connection = &mut self.db_pool.get().unwrap();

        let post_keeper_res: Result<Option<PostKeeper>, diesel::result::Error> = post_keepers.select(PostKeeper::as_select())
            .filter(pioki_id.eq(user_id).and(post_id_col.eq(post_id)))
            .first::<PostKeeper>(connection)
            .optional();

        return post_keeper_res
    }

    pub fn get_all_kept_post_from_user(&self, user_id: String) -> Result<Vec<(PostKeeper, (Post, User))>, diesel::result::Error>{
        use crate::schema::posts::dsl::{posts,id as post_id,creator_id};
        use crate::schema::users::dsl::{pioki_id as user_pioki_id_col,users};
        use crate::schema::post_keepers::dsl::{post_keepers,pioki_id,post_id as post_id_of_post_keepers,created_at};
        let connection = &mut self.db_pool.get().unwrap();

        let keeps:  Result<Vec<(PostKeeper, (Post,User))>, diesel::result::Error> = post_keepers
            .order(created_at.desc())
            .inner_join(posts.on(post_id.eq(post_id_of_post_keepers)).inner_join(users.on(user_pioki_id_col.eq(creator_id))))
            .filter(pioki_id.eq(user_id))
            .select((PostKeeper::as_select(), (Post::as_select(), User::as_select())))
            .load::<(PostKeeper, (Post,User))>(connection);

        keeps
    }

    pub fn pass_post(&self, user_id: String, post_id: i32) -> Result<(), diesel::result::Error>{
        use crate::schema::post_keepers::dsl::{post_keepers,pioki_id,post_id as post_id_col};
        use crate::schema::keep_and_pass_along_logs::dsl::keep_and_pass_along_logs;
        use crate::schema::posts::dsl::{posts,quota_left};
        let connection = &mut self.db_pool.get().unwrap();

        let tx = connection.transaction::<_,diesel::result::Error,_>(|conn|{

            let removal: QueryResult<usize> = diesel::delete(post_keepers.filter(pioki_id.eq(pioki_id).and(post_id_col.eq(post_id)))).execute(conn);
            if removal.is_err(){
                return Err(diesel::result::Error::RollbackTransaction)
            }

            if removal.unwrap() < 1{
                return Err(diesel::result::Error::RollbackTransaction)
            }

            let quota_updation: Result<_, _> = diesel::update(posts.find(post_id)).set(quota_left.eq(quota_left + 1)).execute(conn);
            if quota_updation.is_err(){
                return Err(diesel::result::Error::RollbackTransaction)
            }


            let log_item = NewKeepAndPassAlongLog{pioki_id: user_id.to_string(),post_id,is_kept: false};
            let log_insertion = diesel::insert_into(keep_and_pass_along_logs::table())
                        .values(&log_item)
                        .returning(KeepAndPassAlongLog::as_returning())
                        .get_result::<KeepAndPassAlongLog>(conn);

            if log_insertion.is_err(){
                return Err(diesel::result::Error::RollbackTransaction)
            }


            
            return Ok(())
        });

        tx
    }

    pub fn keep_post(&self, user_id: String, post_id: i32) -> Result<PostKeeper,PostKeepingError>{
        use crate::schema::keep_and_pass_along_logs::dsl::{keep_and_pass_along_logs};
        use crate::schema::post_keepers::dsl::{post_keepers,post_id as post_id_col,pass_along_at};
        use crate::schema::posts::dsl::*;
        use crate::schema::users::dsl::{users,coin_amount,pioki_id,oauth_display_name};
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
                    let post_owner_id = post.creator_id.to_string();
                    let count_res: Result<i64,diesel::result::Error> = post_keepers.select(count_star()).filter(post_id_col.eq(post_id).and(pass_along_at.is_null())).first::<i64>(conn);
                    if let Ok(keep_count) = count_res{
                        if keep_count as i32 >= post.origin_quota_limit{
                            return Err(PostKeepingError::NoMoreQuota) // could be any error tho, but i am not going to extract later . . . as I will always determine this failure by out-of-quota case
                        }

                        // post is keep-able for now
                        // calling insert here

                        let post_keeper_insert_item = NewPostKeeper{pioki_id: &user_id,post_id};
                        let log_item = NewKeepAndPassAlongLog{pioki_id: user_id.to_string(),post_id,is_kept: true};

                        let pk_insertion = diesel::insert_into(post_keepers::table())
                        .values(&post_keeper_insert_item)
                        .returning(PostKeeper::as_returning())
                        .get_result::<PostKeeper>(conn);

                        
                        let log_insertion = diesel::insert_into(keep_and_pass_along_logs::table())
                        .values(&log_item)
                        .returning(KeepAndPassAlongLog::as_returning())
                        .get_result::<KeepAndPassAlongLog>(conn);

                        if post_owner_id != user_id{ // Prevent self spamming, while still allowing to keep yourself message
                            let has_user_kept_this_post_before = self.check_if_user_ever_keep_this_post(user_id.to_string(), post_id);
                            if !has_user_kept_this_post_before{
                                // first time keeping this post
                                // give the point to the post owner
                                // let post_owner_id = post.creator_id.to_string();
    
                                let coin_updation: Result<_, _> = diesel::update(users.filter(pioki_id.eq(post_owner_id.to_string()))).set(coin_amount.eq(coin_amount + BigDecimal::from_i8(1).unwrap())).execute(conn);
                                if coin_updation.is_err(){
                                    return Err(PostKeepingError::RollbackError)
                                }
                            }
                        }

                        // lastly decrease the quota left
                        let quota_updation: Result<_, _> = diesel::update(posts.find(post_id)).set(quota_left.eq(post.origin_quota_limit - (keep_count as i32 + 1))).execute(conn); // post.origin_quota_limit - (keep_count as i32 + 1) for re-stamping intregity by the actual counted result
                        if quota_updation.is_err() || log_insertion.is_err() || pk_insertion.is_err(){
                            // if any error happen to quota updating or log_insertion or pk_insertion -> Rollback
                            // could catch them each right after each operation
                            // later on this would run concurrently :TODO
                            return Err(PostKeepingError::RollbackError)
                        }
                        Ok(pk_insertion.unwrap())
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