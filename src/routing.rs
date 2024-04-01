use std::rc::Rc;

use actix_web::web;
use crate::{db_connection::get_connection_pool, domains::repositories::repositories::Repositories, repository, services::{friends::{list_friend, list_pending_friend_requests, send_friend_request}, me::main::{list_kept_post_ids, list_kept_posts}, posts::main::{check_if_post_is_already_owned, create_post, keep_post, list_user_posts, pass_post}, users::{create_user, get_user, get_users}}};

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
            web::scope("/users")
            .service(
                web::resource("")
                .route(web::get().to(get_users)) // api/user
                .route(web::post().to(create_user)) // api/user
            )
            .service(
                web::resource("/{user_id}")
                .route(web::get().to(get_user))
            )
            .service(
                web::resource("/{user_id}/posts")
                .route(web::get().to(list_user_posts))
            )
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
        ).service(
            web::scope("/posts")
            .service(
                web::resource("").
                route(web::post().to(create_post)) // api/posts
            )
            .service(
                web::resource("/{post_id}/keep").
                route(web::post().to(keep_post)) // api/posts/{post_id}/keep           
            )
            .service(
                web::resource("/{post_id}/is_owned").
                route(web::get().to(check_if_post_is_already_owned)) // api/posts/{post_id}/keep           
            )
            .service(
                web::resource("/{post_id}/pass").
                route(web::post().to(pass_post)) // api/posts/{post_id}/keep           
            )
        )
        .service(
            web::scope("/me")
            .service(
                web::resource("/kept_post_ids").
                route(web::get().to(list_kept_post_ids)) // api/posts
            )
            .service(
                web::resource("/kept_posts").
                route(web::get().to(list_kept_posts)) // api/posts
            )
        )
    );
}