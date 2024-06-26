use actix_web::{web::{Data, Path, ReqData}, HttpRequest, HttpResponse, Responder};

use crate::{domains::{inputs::posts::PostLookupWhereClause, repositories::repositories::Repositories}, dtos::{posts::{CreatePostDTO, PostIdParams, PostResponseeDTO}, users::UserIdParams, ResponseToUserEnd}, middlewares::{valid_incoming_source_checker::PortalAuthenticated, PIOKIIdentifierData}, repository, utils::validation::{self, core::throw_error_response_based_on_validation_error_kind}};

pub async fn list_user_posts(
    _: HttpRequest,
    param: Path<UserIdParams>,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {

    match identifier_data{
        Some(identifier) => {
            match repositories.post_repository.find_all(Some(PostLookupWhereClause{user_id: Some(param.user_id.to_string())})){
                Ok(posts) => {
                    let res = posts.iter().map(|post| PostResponseeDTO{
                        id: post.0.id,
                        creator_id: post.0.creator_id.to_owned(),
                        origin_quota_limit: post.0.origin_quota_limit,
                        quota_left: post.0.quota_left,
                        content: None, //post.0.content.to_owned()
                        created_at: post.0.created_at,
                        updated_at: post.0.updated_at,
                        spoiler_header: post.0.spoiler_header.clone(),
                        user: crate::dtos::users::UserResponseDTO { id: post.1.id, pioki_id: post.1.pioki_id.to_owned(), is_active: post.1.is_active, created_at: post.1.created_at }

                    }).collect::<Vec<PostResponseeDTO>>();
                    let mut res_to_end = ResponseToUserEnd::default();
                    res_to_end.message = Some("success".into());
                    res_to_end.data = Some(res);
                    HttpResponse::Ok().json(res_to_end)
                },
                Err(_) => todo!(),
            }
        },
        None => HttpResponse::BadGateway().body("Bad incoming source"),
    }
}

pub async fn create_post(
    _: HttpRequest,
    body: String,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {

    if identifier_data.is_none(){
        return HttpResponse::BadGateway().body("Bad incoming source")
    }

    let dto = validation::core::serialize_body_into_struct::<CreatePostDTO>(&body);
    match dto{
        Ok(input) => {
            match repositories.post_repository.create_post(identifier_data.unwrap().id.to_string(), input){
                Ok(created_post) => HttpResponse::Ok().json(ResponseToUserEnd::only_this_message("success").with_data(created_post)),
                Err(e) => HttpResponse::InternalServerError().json(ResponseToUserEnd::<()>::only_this_message(e.to_string().as_str())),
            }
        },
        Err(ekind) => throw_error_response_based_on_validation_error_kind(ekind)
    }
    
}


pub async fn keep_post(
    _: HttpRequest,
    param: Path<PostIdParams>,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {

    match identifier_data{
        Some(identifier) => {
            let i32_post_id: Result<i32,_> = param.post_id.parse();
            if let Ok(post_id) = i32_post_id {
                let post_keeping_opearion = repositories.post_repository.keep_post(identifier.id.to_string(),post_id);
                match post_keeping_opearion{
                    Ok(pk) => HttpResponse::Ok().json(ResponseToUserEnd::only_this_message("success").with_data(pk)),
                    Err(e) => match e {
                        repository::posts::PostKeepingError::AlreadyInteractedError => HttpResponse::BadRequest().json(ResponseToUserEnd::<()>::only_this_message("You've once interacted with this post already")),
                        repository::posts::PostKeepingError::RollbackError => HttpResponse::BadRequest().json(ResponseToUserEnd::<()>::only_this_message("Lost the chance to keep post due to race condition")),
                        repository::posts::PostKeepingError::NoMoreQuota => HttpResponse::BadRequest().json(ResponseToUserEnd::<()>::only_this_message("Quota has been already depleted")),
                        repository::posts::PostKeepingError::DatabaseError(_) => HttpResponse::InternalServerError().json(ResponseToUserEnd::<()>::only_this_message("Something has gone wrong with the database")),
                    },
                }
            }else{
                return HttpResponse::BadRequest().body("Invalid post_id in params")
            }
        },
        None => HttpResponse::BadGateway().body("Bad incoming source"),
    }
}

pub async fn check_if_post_is_already_owned(
    _: HttpRequest,
    param: Path<PostIdParams>,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {

    if identifier_data.is_none(){
        return HttpResponse::BadGateway().body("Bad incoming source")
    }

    let i32_post_id: Result<i32,_> = param.post_id.parse();
    if let Ok(post_id) = i32_post_id {
        let post_keep_res = repositories.post_repository.is_owned(identifier_data.unwrap().id.to_string(), post_id);
        match post_keep_res{
            Ok(post_keep_opt) => match post_keep_opt{
                Some(post_keep) => HttpResponse::Ok().json(ResponseToUserEnd::only_this_message("success").with_data(post_keep)),
                None => HttpResponse::Ok().json(ResponseToUserEnd::<()>::only_this_message("success")),
            },
            Err(_) => HttpResponse::InternalServerError().body("Something has gone wrong . . ."),
        }

    }else{
        return HttpResponse::BadRequest().body("Invalid post_id in params")
    }
}

pub async fn pass_post(
    _: HttpRequest,
    param: Path<PostIdParams>,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {

    if identifier_data.is_none(){
        return HttpResponse::BadGateway().body("Bad incoming source")
    }
    

    let i32_post_id: Result<i32,_> = param.post_id.parse();
    if let Ok(post_id) = i32_post_id {
        let operation_res = repositories.post_repository.pass_post(identifier_data.unwrap().id.to_string(), post_id);
        match operation_res{
            Ok(_) => return HttpResponse::Ok().json(ResponseToUserEnd::<()>::only_this_message("success")),
            Err(e) => match e{
                diesel::result::Error::RollbackTransaction => return HttpResponse::BadRequest().json(ResponseToUserEnd::<()>::only_this_message("failed passing along the post")),
                _ => return HttpResponse::InternalServerError().body("Something has gone wrong . . ."),
            },
        }
    }
    else{
        return HttpResponse::BadRequest().body("Invalid post_id in params")
    }
}