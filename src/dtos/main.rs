use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Default)]
pub struct ResponseToUserEnd{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>, // escaped reserve word

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors_by_field: Option<Value>
}

impl ResponseToUserEnd{
    pub fn only_this_message(msg: &str) -> Self{
        let mut preself = Self::default();
        preself.message = Some(msg.to_string());
        preself
    }
}