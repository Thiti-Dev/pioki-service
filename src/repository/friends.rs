use std::{borrow::Borrow, collections::HashMap, ops::Deref, rc::Rc};

use diesel::{associations::HasTable, prelude::*, sql_query, sql_types::Text};
use crate::{db_connection::DbPool, dtos::friends::RelationshipStatus, models::{Friend, User}, schema::users};

use super::users::UserRepository;

#[derive(Clone)]
pub struct FriendRepository{
    pub db_pool: Rc<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>>>,
    pub user_repository: UserRepository
}

impl FriendRepository{
    pub fn create_friend_request(&self,from_id: &str,to_id: &str) -> Result<Friend, diesel::result::Error>{
        use crate::schema::friends::dsl::friends;

        let connection = &mut self.db_pool.get().unwrap();
        let new_friend_request = Friend{pioki_id: from_id.to_string(), pioki_friend_id: to_id.to_string(),is_blocked: false,aka: None,created_at:None,updated_at:None};
        diesel::insert_into(friends::table())
        .values(&new_friend_request)
        .returning(Friend::as_returning())
        .get_result(connection)
    }

    pub fn list_pending_friend_request(&self, user_id: &str) -> Result<Vec<(Friend, User)>, diesel::result::Error>{
        let connection = &mut self.db_pool.get().unwrap();
        let friends_query = sql_query(format!("
        SELECT DISTINCT f1.*
        FROM friends f1
        WHERE pioki_friend_id = '{}'
        AND pioki_id NOT IN (
            SELECT DISTINCT pioki_friend_id
            FROM friends
            WHERE pioki_id = '{}'
            AND pioki_friend_id = f1.pioki_id
        );
        "
        ,user_id,user_id))
        // .bind::<Text,_>(user_id)
        .get_results::<Friend>(connection);

        match friends_query{
            Ok(friends) => {
                let friend_ids = friends.iter().map(|friend| friend.pioki_id.to_string()).collect::<Vec<String>>();
                let users = self.user_repository.get_users_from_ids(&friend_ids);

                let mut mapped_user_by_id = HashMap::new() as HashMap<String, User>;
                users.iter().for_each(|user| {
                    mapped_user_by_id.insert(user.pioki_id.to_string(), user.clone());
                });

                // let res: Vec<(Friend, User)> = Vec::new() as Vec<(Friend,User)>;

                let res = friends.iter().map( move |friend| (friend.clone(), mapped_user_by_id.get(&String::from(friend.pioki_id.to_string())).unwrap().clone())).collect::<Vec<(Friend,User)>>();
                return Ok(res)
            },
            Err(_) => todo!(),
        }
    }

    pub fn list_friend_of_user(&self,user_id: &str) -> Result<Vec<(Friend, User)>, diesel::result::Error>{
        let connection = &mut self.db_pool.get().unwrap();
        // Join the friends table with itself to find mutual friendships

        let friends_query = sql_query(format!("
            SELECT f2.*
            FROM friends f1
            JOIN friends f2 ON f1.pioki_id = f2.pioki_friend_id AND f1.pioki_friend_id = f2.pioki_id
            WHERE f1.pioki_id = '{}'
        "
        ,user_id))
        .get_results::<Friend>(connection);

        match friends_query{
            Ok(friends) => {
                let friend_ids = friends.iter().map(|friend| friend.pioki_id.to_string()).collect::<Vec<String>>();
                let users = self.user_repository.get_users_from_ids(&friend_ids);
                let mut mapped_user_by_id = HashMap::new() as HashMap<String, User>;
                users.iter().for_each(|user| {
                    mapped_user_by_id.insert(user.pioki_id.to_string(), user.clone());
                });
                let res = friends.iter().map( move |friend| (friend.clone(), mapped_user_by_id.get(&String::from(friend.pioki_id.to_string())).unwrap().clone())).collect::<Vec<(Friend,User)>>();
                return Ok(res)
            },
            Err(_) => todo!(),
        }
    }

    pub fn check_if_user_already_be_friend_with_this_user(&self, user_id: String, target_user_id: String) -> Result<RelationshipStatus,diesel::result::Error>{
        let connection = &mut self.db_pool.get().unwrap();
        let friends_query = sql_query(format!("
            SELECT * FROM friends where (pioki_id = '{}' and pioki_friend_id = '{}') 
            or (pioki_friend_id = '{}' and pioki_id = '{}') 
        "
        ,user_id,target_user_id,user_id,target_user_id))
        .get_results::<Friend>(connection);

        match friends_query{
            Ok(friends) => {
                let length =  friends.len();
                if length == 2{
                    return Ok(RelationshipStatus::Friended)
                }
                else if length == 1{
                    if friends[0].pioki_id == user_id{
                        return Ok(RelationshipStatus::Requested)
                    }else{
                        return Ok(RelationshipStatus::Pending)
                    }
                }else{
                    return Ok(RelationshipStatus::None)
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn remove_friend(&self, user_id: String, target_user_id: String) -> bool{
        use crate::schema::friends::dsl::*;

        let connection = &mut self.db_pool.get().unwrap();
        let removal: QueryResult<usize> = diesel::delete(friends.filter((pioki_id.eq(user_id.to_owned()).and(pioki_friend_id.eq(target_user_id.to_owned()))).or(pioki_id.eq(target_user_id).and(pioki_friend_id.eq(user_id))))).execute(connection);

        return removal.is_ok()
    }
}