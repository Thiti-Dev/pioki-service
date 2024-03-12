use routing::configure_route;
use serde::Serialize;
use actix_web::{ dev::Service as _, middleware::{self, Logger}, App, HttpServer};
use dotenv::dotenv;

mod routing;
mod services;
mod utils;
mod middlewares;
mod repository;
mod db_connection;
mod dtos;
mod models;
mod schema;


#[derive(Serialize)] // Required for serde
struct ProviderPayload {
  email: String,
  id: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // ENV loader
    dotenv().ok();
    //
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                middlewares::identifier_extractor(&req);
                srv.call(req)
            })
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.1")))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap_fn(|req,srv| {
                middlewares::valid_incoming_source_checker::valid_incoming_source_checker(&req);
                srv.call(req)
            })
            .configure(configure_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}