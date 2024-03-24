use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::users::UserResponseDTO;

#[derive(Deserialize,Debug,Validate)]
pub struct CreatePostDTO {
    #[validate(length(min = 1, message = "Content data cannot be empty"))] // limitede to only 32 chars, will be sliced in the insertion process
    pub content: String,
    
    pub quota_limit: u32,

    #[validate(url(message = "The url is invalid"))]
    pub oauth_profile_picture: Option<String>
}

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

#[derive(Deserialize, Debug)]
pub struct PostIdParams {
    pub post_id: String,
}