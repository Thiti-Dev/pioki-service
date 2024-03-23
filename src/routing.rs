use actix_web::web;
use crate::{db_connection::get_connection_pool, repository, services::{friends::{list_friend, list_pending_friend_requests, send_friend_request}, users::{create_user, get_users}}};

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
    let user_repository = repository::users::UserRepository{
        db_pool: pool.clone()
    };
    let user_repository_web_data = web::Data::new(user_repository.clone());
    let friend_repository = web::Data::new(repository::friends::FriendRepository{
        db_pool: pool.clone(),
        user_repository: user_repository.clone()
    });
    cfg.app_data(app_state).app_data(user_repository_web_data).app_data(friend_repository); // .clone?
    cfg.service(
    web::scope("/api")
        .service(
            web::resource("/users").
            route(web::get().to(get_users)) // api/user
            .route(web::post().to(create_user)) // api/user
        )
        .service(
            web::resource("/users/{send_to_user_id}/send-friend-request").
            route(web::post().to(send_friend_request))
        )
        .service(
            web::scope("/friends")
            .service(
                web::resource("").
                route(web::get().to(list_friend)) // api/friends
            )
            .service(
                web::resource("/pending").
                route(web::get().to(list_pending_friend_requests)) // api/friends/pending           
            )
        )
    );
}