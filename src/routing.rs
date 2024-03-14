use actix_web::web;
use crate::{db_connection::get_connection_pool, repository, services::users::{create_user, get_users}};

pub struct AppState{
    pub suspicious: bool
}

pub fn configure_route(cfg: &mut web::ServiceConfig) {
    let pool: r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>> = get_connection_pool();
    // let db_pool_data = web::Data::new(
    //     pool.clone()
    // );
    let app_state: web::Data<AppState> = web::Data::new(AppState{
        suspicious: false
    });
    let user_repository = web::Data::new(repository::users::UserRepository{
        db_pool: pool
    });
    cfg.app_data(app_state).app_data(user_repository); // .clone?
    cfg.service(
    web::scope("/api")
        .service(
            web::resource("/users").
            route(web::get().to(get_users))
            .route(web::post().to(create_user))
        )
    );
}