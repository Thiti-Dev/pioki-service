use actix_web::web::{Data, ReqData};
use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::middlewares::PIOKIIdentifierData;
use crate::models::User;
use crate::repository;

pub async fn get_users(_: HttpRequest,user_repository: Data<repository::users::UserRepository>) -> impl Responder {
    let users = user_repository.get_users();

    // let response = users.iter().map(|user| UserResponseDTO{
    //     id: user.id,
    //     pioki_id: user.pioki_id.clone(),
    //     is_active: user.is_active,
    //     created_at: user.created_at.to_string()
    // }).collect::<Vec<UserResponseDTO>>();
    
    HttpResponse::Ok().json(users)
}

pub async fn create_user(_: HttpRequest,identifier_data: Option<ReqData<PIOKIIdentifierData>>,user_repository: Data<repository::users::UserRepository>) -> impl Responder {
    match identifier_data{
        Some(identifier) => {
            // if identifier found from header, meaning that this came from pioki-frontend
            match user_repository.create_user(identifier.id.to_string().as_str()){
                Ok(created_user) => HttpResponse::Created().json(created_user),
                Err(diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                )) => {
                    HttpResponse::Conflict().body("Identifier does already exist")
                },
                Err(_) => {
                    HttpResponse::InternalServerError().body("Something went wrong creating user")
                },
            }
        }
        None => {
            HttpResponse::BadGateway().body("Bad incoming source")
        },
    }
}