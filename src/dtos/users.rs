use serde::Serialize;


#[derive(Serialize)]
pub struct UserResponseDTO{
    // #[serde(rename = "user_id")]
    pub id:i32,
    pub pioki_id: String,
    pub is_active: bool,
    pub created_at: String
}