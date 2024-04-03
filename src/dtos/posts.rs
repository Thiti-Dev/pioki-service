use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::{Post, PostKeeper, User};

use super::users::UserResponseDTO;

#[derive(Deserialize,Debug,Validate)]
pub struct CreatePostDTO {
    #[validate(length(min = 1, message = "Content data cannot be empty"))] // limitede to only 32 chars, will be sliced in the insertion process
    pub content: String,
    
    pub quota_limit: u32,

    #[validate(url(message = "The url is invalid"))]
    pub oauth_profile_picture: Option<String>,

    #[validate(length(min = 3,max = 50, message = "Should have atleast 3 characters and at most 50 characters"))]
    pub spoiler_header: Option<String>
}

#[derive(Serialize)]
pub struct PostResponseeDTO{
    // #[serde(rename = "user_id")]
    pub id:i32,
    pub creator_id: String,
    pub spoiler_header: Option<String>,
    pub content: Option<String>,
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

#[derive(Serialize)]
pub struct ListKeptPostsResponseDTO{
    // #[serde(rename = "user_id")]
    pub post_data: Post,
    pub creator_data: User,
    pub keep_data: PostKeeper,
}