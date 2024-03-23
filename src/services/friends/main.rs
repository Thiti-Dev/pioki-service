use actix_web::{web::{Data, Path, ReqData}, HttpRequest, HttpResponse, Responder};

use crate::{dtos::{friends::{ListFriendResponseDTO, PendingFriendResponseDTO}, users::SendFriendRequestParams, ResponseToUserEnd}, middlewares::{valid_incoming_source_checker::PortalAuthenticated, PIOKIIdentifierData}, repository};

pub async fn send_friend_request(
    _: HttpRequest,
    param: Path<SendFriendRequestParams>,
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    user_repository: Data<repository::users::UserRepository>,
    friend_repository: Data<repository::friends::FriendRepository>,
    _:PortalAuthenticated) -> impl Responder {
        
    match identifier_data{
        Some(identifier) => {
            match user_repository.get_user(&param.send_to_user_id){
                Ok(_user) => match _user{
                    Some(_) => match friend_repository.create_friend_request(&identifier.id, &param.send_to_user_id){
                        Ok(created_friend_request) => HttpResponse::Ok().json(created_friend_request),
                        //insert or update on table "friends" violates foreign key constraint "friends_pioki_id_fkey"
                        Err(e) => match e {
                            diesel::result::Error::DatabaseError(dberror, _) => match dberror{
                                diesel::result::DatabaseErrorKind::UniqueViolation => HttpResponse::BadRequest().json(ResponseToUserEnd::only_this_message("Request is already in pending state")),
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
                        let mut res = ResponseToUserEnd::default();
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
    friend_repository: Data<repository::friends::FriendRepository>,
    _:PortalAuthenticated) -> impl Responder {

    match identifier_data{
        Some(identifier) => {
            match friend_repository.list_pending_friend_request(&identifier.id){
                Ok(res) => {
                    let res = res.iter().map(|ele| PendingFriendResponseDTO{
                        id: ele.1.id,
                        oauth_display_name: ele.1.oauth_display_name.to_owned(),
                        oauth_profile_picture: ele.1.oauth_profile_picture.to_owned(),
                        pioki_id: ele.1.pioki_id.to_owned(),
                        requested_at: ele.0.created_at
                    }).collect::<Vec<PendingFriendResponseDTO>>();
                    HttpResponse::Ok().json(res)
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
    identifier_data: Option<ReqData<PIOKIIdentifierData>>,
    friend_repository: Data<repository::friends::FriendRepository>,
    _:PortalAuthenticated) -> impl Responder {
    match identifier_data{
        Some(identifier) => {
            match friend_repository.list_friend_of_user(&identifier.id){
                Ok(res) => {
                    let res = res.iter().map(|ele| ListFriendResponseDTO{
                        id: ele.1.id,
                        oauth_display_name: ele.1.oauth_display_name.to_owned(),
                        oauth_profile_picture: ele.1.oauth_profile_picture.to_owned(),
                        pioki_id: ele.0.pioki_friend_id.to_owned()
                    }).collect::<Vec<ListFriendResponseDTO>>();
                    HttpResponse::Ok().json(res)                    
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