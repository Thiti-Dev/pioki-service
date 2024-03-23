use chrono::NaiveDateTime;
use serde::Serialize;

use super::users::UserResponseDTO;

#[derive(Serialize)]
pub struct PostResponseeDTO{
    // #[serde(rename = "user_id")]
    pub id:i32,
    pub creator_id: String,
    pub content: String,
    pub origin_quota_limit: i32,
    pub quota_left: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub user: UserResponseDTO
}