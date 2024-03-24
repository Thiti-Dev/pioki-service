use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Default, Clone)]
pub struct ResponseToUserEnd<T = ()>{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>, // escaped reserve word

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors_by_field: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}

impl<T> ResponseToUserEnd<T>
where T: Default
{
    pub fn only_this_message(msg: &str) -> Self{
        let mut preself = Self::default();
        preself.message = Some(msg.to_string());
        preself
    }

    pub fn with_data(&mut self, data: T) -> &mut Self{
        self.data = Some(data);
        self
    }
}