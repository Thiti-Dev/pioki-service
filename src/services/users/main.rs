use actix_web::web::{Data, ReqData};
use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::domains::repositories::repositories::Repositories;
use crate::dtos::users::CreateUserDTO;
use crate::middlewares::valid_incoming_source_checker::PortalAuthenticated;
use crate::middlewares::PIOKIIdentifierData;
use crate::repository;
use crate::utils::validation;
use crate::utils::validation::core::throw_error_response_based_on_validation_error_kind;

pub async fn get_users(_: HttpRequest,repositories: Data<Repositories>) -> impl Responder {
    let users = repositories.user_repository.get_users();

    // let response = users.iter().map(|user| UserResponseDTO{
    //     id: user.id,
    //     pioki_id: user.pioki_id.clone(),
    //     is_active: user.is_active,
    //     created_at: user.created_at.to_string()
    // }).collect::<Vec<UserResponseDTO>>();
    
    HttpResponse::Ok().json(users)
}

pub async fn create_user(_: HttpRequest,body: String ,identifier_data: Option<ReqData<PIOKIIdentifierData>>,repositories: Data<Repositories>,_:PortalAuthenticated) -> impl Responder {
    //println!("{}", body);
    let dto = validation::core::serialize_body_into_struct::<CreateUserDTO>(&body);

    match dto{
        Ok(create_user_dto) => {
            match identifier_data{
                Some(identifier) => {
                    // if identifier found from header, meaning that this came from pioki-frontend
                    match repositories.user_repository.create_user(identifier.id.to_string().as_str(),&create_user_dto.oauth_display_name[..if create_user_dto.oauth_display_name.len() > 32 {32} else {create_user_dto.oauth_display_name.len()}],create_user_dto.oauth_profile_picture.as_deref()){
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
        },
        Err(ekind) => throw_error_response_based_on_validation_error_kind(ekind)
    }
}