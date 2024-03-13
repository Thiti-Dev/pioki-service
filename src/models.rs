use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;


#[derive(Queryable, Selectable, Insertable,Serialize)]
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