use std::future;

use google_oauth::AsyncClient;
use reqwest::Client;
use routing::get_auth_web_scope;
use serde_json::{Value};
use serde::{Serialize, Deserialize};
use actix_web::{ dev::{Payload, Service as _}, get, middleware::{self, Logger}, post, web, App, FromRequest, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

mod routing;
mod services;
mod utils;
mod middlewares;



#[derive(Serialize)] // Required for serde
struct ProviderPayload {
  email: String,
  id: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // ENV loader
    dotenv().ok();
    let _next_jwt_secret = std::env::var("NEXT_JWT_SECRET").expect("NEXT_JWT_SECRET must be set.");
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
            .service(get_auth_web_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}