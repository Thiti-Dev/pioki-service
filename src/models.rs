use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;


#[derive(Queryable, Selectable, Insertable,Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,

    pub pioki_id: String,

    #[serde(skip_serializing)]
    pub is_active: bool,

    pub created_at: NaiveDateTime,
    
    pub updated_at: NaiveDateTime,
}