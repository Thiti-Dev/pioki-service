use std::collections::HashMap;
use std::fmt::Pointer;

use actix_web::HttpResponse;
use actix_web::Responder;
use serde::de::DeserializeOwned;
use serde_json::Error;
use validator::ValidationErrors;
use validator::Validate;

use crate::dtos::ResponseToUserEnd;


#[derive(Debug)]
pub struct SerializationErrorData{
    pub message: String
}

impl SerializationErrorData{
    fn new(msg: String) -> Self{
        Self { message: msg }
    }
}

#[derive(Debug)]
pub struct ValidationErrorData{
    pub errors: ValidationErrors
}

impl ValidationErrorData{
    fn new(errors: ValidationErrors) -> Self{
        Self { errors }
    }

    #[deprecated(note = "Don't have to use this, do ValidationErrors.field_errors() instead")]
    pub fn into_hashmap(self) -> HashMap<String, String>{
        let mut errors_list: HashMap<String, String> = HashMap::new();
        for (field, errors) in self.errors.errors() {
            errors_list.insert(field.to_string(), format!("{:?}", errors));
        }
        errors_list
    }
}

#[derive(Debug)]
pub enum ValidationErrorKind {
    SerializationError(SerializationErrorData),
    ValidationError(ValidationErrorData),
    // Add other error types as needed
}

/// The T should derive Validate otherwise it won't be callable
pub fn serialize_body_into_struct<T: DeserializeOwned + Validate>(plain_body: &str) -> Result<T, ValidationErrorKind>{
    let create_user_dto_result: Result<T, Error> = serde_json::from_str(plain_body);

    match create_user_dto_result{
        Ok(dto) => {
            // validate
            match dto.validate(){
                Ok(_) => Ok(dto),
                Err(e) => Err(ValidationErrorKind::ValidationError(ValidationErrorData::new(e))),
            }
        },
        Err(e) => Err(ValidationErrorKind::SerializationError(SerializationErrorData::new(e.to_string()))),
    }
}

pub fn throw_error_response_based_on_validation_error_kind(ekind: ValidationErrorKind) -> HttpResponse{
    match ekind{
        ValidationErrorKind::SerializationError(e) =>  HttpResponse::UnprocessableEntity().body(e.message),
        ValidationErrorKind::ValidationError(e) => {
            let json_string = serde_json::to_string(&e.errors.field_errors()).unwrap();
            let errors = serde_json::from_str(&json_string).unwrap();
            let mut res = ResponseToUserEnd::<()>::default();
            res.validation_errors_by_field = errors;
            res.r#type = Some("Validation error".to_string());
            res.data = None;
            // HttpResponse::BadRequest().content_type("application/json").body(json_string)
            HttpResponse::BadRequest().json(res)
        }
    }
}