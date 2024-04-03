use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct PendingFriendResponseDTO{
    // #[serde(rename = "user_id")]
    pub id:i32,
    pub pioki_id: String,
    pub oauth_display_name: String,
    pub oauth_profile_picture:  Option<String>,
    pub requested_at: Option<NaiveDateTime>,
    pub coin_owned: BigDecimal
}

#[derive(Serialize)]
pub struct ListFriendResponseDTO{
    // #[serde(rename = "user_id")]
    pub id:i32,
    pub pioki_id: String,
    pub oauth_display_name: String,
    pub oauth_profile_picture:  Option<String>,
    pub coin_owned: BigDecimal
}

#[derive(Serialize,Default)]
pub struct RelationshipCheckResponseDTO{
    // #[serde(rename = "user_id")]
    pub status: String
}

pub enum RelationshipStatus{
    Requested, // U1 Requested to U2
    Friended, // U1 and U2 already be friend
    Pending, // U2 requested to U! (in U1's pending state)
    None
}

impl ToString for RelationshipStatus {
    fn to_string(&self) -> String {
        match self {
            RelationshipStatus::Requested => String::from("requested"),
            RelationshipStatus::Friended => String::from("friended"),
            RelationshipStatus::Pending => String::from("pending"),
            RelationshipStatus::None => String::from("none"),
        }
    }
}
