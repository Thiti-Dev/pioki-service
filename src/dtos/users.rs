use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct UserResponseDTO{
    // #[serde(rename = "user_id")]
    pub id:i32,
    pub pioki_id: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime
}

#[derive(Deserialize,Debug,Validate)]
pub struct CreateUserDTO {
    #[validate(length(min = 3, message = "Should have atleast 3 characters"))] // limitede to only 32 chars, will be sliced in the insertion process
    pub oauth_display_name: String,

    #[validate(url(message = "The url is invalid"))]
    pub oauth_profile_picture: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct SendFriendRequestParams {
    pub send_to_user_id: String,
}

#[derive(Deserialize, Debug)]
pub struct UserIdParams {
    pub user_id: String,
}