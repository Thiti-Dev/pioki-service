use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct PendingFriendResponseDTO{
    // #[serde(rename = "user_id")]
    pub id:i32,
    pub pioki_id: String,
    pub oauth_display_name: String,
    pub oauth_profile_picture:  Option<String>,
    pub requested_at: Option<NaiveDateTime>
}

#[derive(Serialize)]
pub struct ListFriendResponseDTO{
    // #[serde(rename = "user_id")]
    pub id:i32,
    pub pioki_id: String,
    pub oauth_display_name: String,
    pub oauth_profile_picture:  Option<String>
}