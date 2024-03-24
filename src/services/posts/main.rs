use actix_web::{web::{Data, Path, ReqData}, HttpRequest, HttpResponse, Responder};

use crate::{domains::{inputs::posts::PostLookupWhereClause, repositories::repositories::Repositories}, dtos::{posts::{PostIdParams, PostResponseeDTO}, users::UserIdParams, ResponseToUserEnd}, middlewares::{valid_incoming_source_checker::PortalAuthenticated, PIOKIIdentifierData}, repository};

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
                    println!("{}", posts.len());
                    let res = posts.iter().map(|post| PostResponseeDTO{
                        id: post.0.id,
                        creator_id: post.0.creator_id.to_owned(),
                        origin_quota_limit: post.0.origin_quota_limit,
                        quota_left: post.0.quota_left,
                        content: post.0.content.to_owned(),
                        created_at: post.0.created_at,
                        updated_at: post.0.updated_at,
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
                    Ok(pk) => HttpResponse::Ok().json(pk),
                    Err(e) => match e {
                        repository::posts::PostKeepingError::AlreadyInteractedError => HttpResponse::BadRequest().json(ResponseToUserEnd::<()>::only_this_message("You've once interacted with this post already")),
                        repository::posts::PostKeepingError::RollbackError => HttpResponse::BadRequest().json(ResponseToUserEnd::<()>::only_this_message("Lost the chance to keep post due to race condition"))
                        ,
                    },
                }
            }else{
                return HttpResponse::BadRequest().body("Invalid post_id in params")
            }
        },
        None => HttpResponse::BadGateway().body("Bad incoming source"),
    }
}