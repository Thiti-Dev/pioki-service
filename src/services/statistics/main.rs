use actix_web::{web::Data, HttpRequest, HttpResponse, Responder};

use crate::{domains::repositories::repositories::Repositories, dtos::ResponseToUserEnd, middlewares::valid_incoming_source_checker::PortalAuthenticated};

pub async fn get_statistic_data(
    _: HttpRequest,
    repositories: Data<Repositories>) -> impl Responder {

    let data = repositories.statistic_repository.get_statistic_data();
    HttpResponse::Ok().json(ResponseToUserEnd::only_this_message("success").with_data(data))
    
}
