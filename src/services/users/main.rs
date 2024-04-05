use actix_web::web::{Data, Path, ReqData};
use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::domains::repositories::repositories::Repositories;
use crate::dtos::users::{CreateUserDTO, UserIdParams};
use crate::dtos::ResponseToUserEnd;
use crate::middlewares::valid_incoming_source_checker::PortalAuthenticated;
use crate::middlewares::PIOKIIdentifierData;
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
    
    HttpResponse::Ok().json(ResponseToUserEnd::only_this_message("success").with_data(users))
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
                        Ok(created_user) => HttpResponse::Created().json(ResponseToUserEnd::only_this_message("success").with_data(created_user)),
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

pub async fn get_user(_: HttpRequest,body: String,param: Path<UserIdParams>,repositories: Data<Repositories>) -> impl Responder {
    let user_result = repositories.user_repository.get_user(&param.user_id);
    
    match user_result {
        Ok(user_opt) => {
            if let Some(user) = user_opt{
                return HttpResponse::Ok().json(ResponseToUserEnd::only_this_message("success").with_data(user))
            }else{
                return HttpResponse::NotFound().json(ResponseToUserEnd::<()>::only_this_message("Not found"))
            }
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string())
    }
}