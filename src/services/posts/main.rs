use actix_web::{web::{Data, Path, ReqData}, HttpRequest, HttpResponse, Responder};

use crate::{domains::inputs::posts::PostLookupWhereClause, dtos::{posts::PostResponseeDTO, users::UserIdParams}, middlewares::{valid_incoming_source_checker::PortalAuthenticated, PIOKIIdentifierData}, repository};

pub async fn list_user_posts(
    _: HttpRequest,
    param: Path<UserIdParams>,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    post_repository: Data<repository::posts::PostRepository>,
    _:PortalAuthenticated) -> impl Responder {

    match identifier_data{
        Some(identifier) => {
            match post_repository.find_all(Some(PostLookupWhereClause{user_id: Some(param.user_id.to_string())})){
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
                    HttpResponse::Ok().json(res)
                },
                Err(_) => todo!(),
            }
        },
        None => HttpResponse::BadGateway().body("Bad incoming source"),
    }
}