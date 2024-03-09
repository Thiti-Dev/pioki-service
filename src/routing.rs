use actix_web::{web, Scope};

use crate::services::authentication::test;


pub fn get_auth_web_scope() -> Scope{
    web::scope("/api")
    .service(
        web::resource("/auth").
        route(web::get().to(test))
    )
}