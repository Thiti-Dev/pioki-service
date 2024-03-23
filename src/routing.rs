use std::rc::Rc;

use actix_web::web;
use crate::{db_connection::get_connection_pool, domains::repositories::repositories::Repositories, repository, services::{friends::{list_friend, list_pending_friend_requests, send_friend_request}, posts::main::list_user_posts, users::{create_user, get_users}}};

pub struct AppState{
    pub suspicious: bool
}

pub fn configure_route(cfg: &mut web::ServiceConfig) {
    let pool: r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>> = get_connection_pool();
    let db_pool: Rc<r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::prelude::PgConnection>>> = Rc::new(pool);
    let app_state: web::Data<AppState> = web::Data::new(AppState{
        suspicious: false
    });
    let user_repository = repository::users::UserRepository{
        db_pool:Rc::clone(&db_pool)
    };
    let friend_repository = repository::friends::FriendRepository{
        db_pool:Rc::clone(&db_pool),
        user_repository: user_repository.clone()
    };
    let post_repository = repository::posts::PostRepository{
        db_pool:Rc::clone(&db_pool)
    };

    let app_repositories = web::Data::new(Repositories{
        friend_repository,
        user_repository,
        post_repository
    });

    // TODO: Bundle repos into single struct which has fields that contains repository
    cfg.app_data(app_state).app_data(app_repositories);
    cfg.service(
    web::scope("/api")
        .service(
            web::resource("/users").
            route(web::get().to(get_users)) // api/user
            .route(web::post().to(create_user)) // api/user
        )
        .service(
            web::resource("/users/{user_id}/posts").
            route(web::get().to(list_user_posts))
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