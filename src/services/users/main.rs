use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, Responder};

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