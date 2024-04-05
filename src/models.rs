use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use diesel::sql_types::*;


#[derive(Queryable, Selectable, Insertable,Serialize,Clone,Default)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,

    pub pioki_id: String,

    pub oauth_display_name: String,

    pub oauth_profile_picture: Option<String>,

    #[serde(skip_serializing)]
    pub is_active: bool,

    pub created_at: NaiveDateTime,
    
    pub updated_at: NaiveDateTime,

    pub coin_amount: BigDecimal
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub pioki_id: &'a str,
    pub oauth_display_name: &'a str,
    pub oauth_profile_picture: Option<&'a str>,
    pub is_active: bool,
}

#[derive(Queryable, Selectable, Insertable,Serialize,QueryableByName,Clone)]
#[diesel(table_name = crate::schema::friends)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Friend {
    pub pioki_id: String,

    pub pioki_friend_id: String,

    pub is_blocked: bool,

    pub aka: Option<String>,

    pub created_at: Option<NaiveDateTime>,
    
    pub updated_at: Option<NaiveDateTime>,
}


#[derive(Queryable, Selectable, Insertable,Serialize,QueryableByName,Clone,PartialEq, Default)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,

    pub creator_id: String,

    pub spoiler_header: Option<String>,

    pub origin_quota_limit: i32,

    pub quota_left: i32,

    pub content: String,

    pub created_at: NaiveDateTime,
    
    pub updated_at: NaiveDateTime,
}

#[derive(QueryableByName,Serialize,Clone,Default)]
#[diesel(table_name = crate::schema::posts)]
pub struct FeedData{
    pub id: i32,

    pub creator_id: String,

    pub spoiler_header: Option<String>,

    pub origin_quota_limit: i32,

    pub quota_left: i32,

    pub content: String,

    pub created_at: NaiveDateTime,
    
    pub updated_at: NaiveDateTime,

    #[sql_type = "Text"]
    pub oauth_display_name : String,

    #[sql_type = "Text"]
    pub oauth_profile_picture : String,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPost {
    pub creator_id: String,

    pub spoiler_header: Option<String>,

    pub origin_quota_limit: i32,

    pub quota_left: i32,

    pub content: String,
}

#[derive(Queryable, Selectable, Insertable,Serialize,QueryableByName,Clone,PartialEq,Default)]
#[diesel(table_name = crate::schema::post_keepers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostKeeper {
    pub id: i32,

    pub pioki_id: String,

    pub post_id: i32,

    pub pass_along_at: Option<NaiveDateTime>,

    pub created_at: NaiveDateTime,
    
    pub updated_at: NaiveDateTime,
}


#[derive(Queryable, Selectable, Insertable,Serialize,QueryableByName,Clone,PartialEq,Default)]
#[diesel(table_name = crate::schema::keep_and_pass_along_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct KeepAndPassAlongLog {
    pub id: i32,

    pub pioki_id: String,

    pub post_id: i32,

    pub is_kept: bool,

    pub created_at: NaiveDateTime,
    
    pub updated_at: NaiveDateTime,
}



#[derive(Insertable)]
#[diesel(table_name = crate::schema::post_keepers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPostKeeper<'a> {
    pub pioki_id: &'a str,
    pub post_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::keep_and_pass_along_logs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewKeepAndPassAlongLog {
    pub pioki_id: String,
    pub post_id: i32,
    pub is_kept: bool
}