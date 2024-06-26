use actix_web::{web::{Data, Path, ReqData}, HttpRequest, HttpResponse, Responder};

use crate::{domains::repositories::repositories::Repositories, dtos::{friends::{ListFriendResponseDTO, PendingFriendResponseDTO}, users::{SendFriendRequestParams, UserIdParams}, ResponseToUserEnd}, middlewares::{valid_incoming_source_checker::PortalAuthenticated, PIOKIIdentifierData}, repository};

pub async fn send_friend_request(
    _: HttpRequest,
    param: Path<SendFriendRequestParams>,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {
        
    match identifier_data{
        Some(identifier) => {
            match repositories.user_repository.get_user(&param.send_to_user_id){
                Ok(_user) => match _user{
                    Some(_) => match repositories.friend_repository.create_friend_request(&identifier.id, &param.send_to_user_id){
                        Ok(created_friend_request) => HttpResponse::Ok().json(created_friend_request),
                        //insert or update on table "friends" violates foreign key constraint "friends_pioki_id_fkey"
                        Err(e) => match e {
                            diesel::result::Error::DatabaseError(dberror, _) => match dberror{
                                diesel::result::DatabaseErrorKind::UniqueViolation => HttpResponse::BadRequest().json(ResponseToUserEnd::<()>::only_this_message("Request is already in pending state")),
                                diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                                    if e.to_string().contains("friends_pioki_id_fkey"){
                                        return HttpResponse::BadRequest().body("Your user_id is invalid")
                                    }
                                    return HttpResponse::BadRequest().body(e.to_string())
                                }
                                _ => todo!(),
                            },
                            _ => HttpResponse::InternalServerError().body("Something might have gone wrong . . ."),
                        },
                    },
                    None => {
                        let mut res = ResponseToUserEnd::<()>::default();
                        res.message = Some(format!("User with id:\"{}\" doesn't exist", param.send_to_user_id));
                        HttpResponse::NotFound().json(res)
                    }
                ,
                },
                Err(_) => HttpResponse::InternalServerError().body("Something might have gone wrong . . ."),
            }
        },
        None => HttpResponse::BadGateway().body("Bad incoming source")
    }
}

pub async fn list_pending_friend_requests(
    _: HttpRequest,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {

    match identifier_data{
        Some(identifier) => {
            match repositories.friend_repository.list_pending_friend_request(&identifier.id){
                Ok(res) => {
                    let res = res.iter().map(|ele| PendingFriendResponseDTO{
                        id: ele.1.id,
                        oauth_display_name: ele.1.oauth_display_name.to_owned(),
                        oauth_profile_picture: ele.1.oauth_profile_picture.to_owned(),
                        pioki_id: ele.1.pioki_id.to_owned(),
                        requested_at: ele.0.created_at,
                        coin_owned: ele.1.coin_amount.to_owned()
                    }).collect::<Vec<PendingFriendResponseDTO>>();
                    HttpResponse::Ok().json(ResponseToUserEnd::only_this_message("success").with_data(res))
                },
                Err(e) => {
                    println!("{}", e.to_string());
                    return HttpResponse::BadGateway().body("Bad incoming source")
                },
            }
        },
        None => HttpResponse::BadGateway().body("Bad incoming source"),
    }
}

pub async fn list_friend(
    _: HttpRequest ,
    param: Path<UserIdParams>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {
        match repositories.friend_repository.list_friend_of_user(&param.user_id){
                Ok(res) => {
                    let res = res.iter().map(|ele| ListFriendResponseDTO{
                        id: ele.1.id,
                        oauth_display_name: ele.1.oauth_display_name.to_owned(),
                        oauth_profile_picture: ele.1.oauth_profile_picture.to_owned(),
                        pioki_id: ele.0.pioki_id.to_owned(),
                        coin_owned: ele.1.coin_amount.to_owned()
                    }).collect::<Vec<ListFriendResponseDTO>>();
                    HttpResponse::Ok().json(ResponseToUserEnd::only_this_message("success").with_data(res))                    
                },
                Err(e) => {
                    return HttpResponse::BadGateway().body("Bad incoming source")
                },
        }
 }

 pub async fn remove_friend(
    _: HttpRequest ,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    param: Path<UserIdParams>,
    repositories: Data<Repositories>,
    _:PortalAuthenticated) -> impl Responder {
        match identifier_data{
            Some(identifier) => {
                let removal = repositories.friend_repository.remove_friend(identifier.id.to_owned(), param.user_id.to_owned());
                if !removal{
                    return HttpResponse::InternalServerError().json(ResponseToUserEnd::<()>::only_this_message("Unknown error has occured")) 
                }

                return HttpResponse::Ok().json(ResponseToUserEnd::<()>::only_this_message("success"))
            },
            None => HttpResponse::BadGateway().body("Bad incoming source"),
        }      
 }