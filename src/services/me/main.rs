use std::collections::HashMap;

use actix_web::{web::{Data, Path, ReqData}, HttpRequest, HttpResponse, Responder};

use crate::{domains::repositories::repositories::Repositories, dtos::{friends::RelationshipCheckResponseDTO, posts::ListKeptPostsResponseDTO, users::UserIdParams, ResponseToUserEnd}, middlewares::{valid_incoming_source_checker::PortalAuthenticated, PIOKIIdentifierData}};

pub async fn list_kept_post_ids(
    _: HttpRequest,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {

    if identifier_data.is_none(){
        return HttpResponse::BadGateway().body("Bad incoming source")
    }

    let kept_posts_res = repositories.post_repository.get_all_kept_post_from_user(identifier_data.unwrap().id.to_string());

    match kept_posts_res {
        Ok(kept_posts) => {
            let mut mapped_owned_post_ids_by_post_id: HashMap<i32, bool> = HashMap::new();

            for post in kept_posts {
                mapped_owned_post_ids_by_post_id.insert(post.0.post_id, true);
            }
            HttpResponse::Ok().json(ResponseToUserEnd::only_this_message("success").with_data(mapped_owned_post_ids_by_post_id))
        },
        Err(_) => HttpResponse::InternalServerError().body("Something has gone wrong . . ."),
    }
}

pub async fn list_kept_posts(
    _: HttpRequest,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {

    if identifier_data.is_none(){
        return HttpResponse::BadGateway().body("Bad incoming source")
    }

    let kept_posts_res = repositories.post_repository.get_all_kept_post_from_user(identifier_data.unwrap().id.to_string());

    match kept_posts_res {
        Ok(kept_posts) => {
            let mut res: Vec<ListKeptPostsResponseDTO> = Vec::new();
            for post in kept_posts {
                res.push(ListKeptPostsResponseDTO{
                    creator_data: post.1.1,
                    post_data: post.1.0,
                    keep_data: post.0
                })
            }

            HttpResponse::Ok().json(ResponseToUserEnd::only_this_message("success").with_data(res))
        },
        Err(_) => HttpResponse::InternalServerError().body("Something has gone wrong . . ."),
    }
}

pub async fn get_relationship_status_with_user(
    _: HttpRequest,
    param: Path<UserIdParams>,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {

    if identifier_data.is_none(){
        return HttpResponse::BadGateway().body("Bad incoming source")
    }

    let relationship_status_res = repositories.friend_repository.check_if_user_already_be_friend_with_this_user(identifier_data.unwrap().id.to_string(), param.user_id.to_string());
    match relationship_status_res {
        Ok(relationship) => HttpResponse::Ok().json(ResponseToUserEnd::only_this_message("success").with_data(RelationshipCheckResponseDTO{status: relationship.to_string()})),
        Err(_) => HttpResponse::InternalServerError().body("Something has gone wrong . . ."),
    }

}