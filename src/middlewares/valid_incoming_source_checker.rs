use std::future::{ready, Ready};

use actix_web::{dev::ServiceRequest, Error, FromRequest, HttpMessage};

use once_cell::sync::Lazy;

/// Lazied because has to wait the dotenv to be initialized first in the main entry
static mut LAZY_PIOKI_ACCESS_KEY: Lazy<String> = Lazy::new(|| {
    // LAZY_PIOKI_ACCESS_KEY should be accessed after dotenv initialization
    std::env::var("PIOKI_SECRET_ACCESS_KEY").expect("PIOKI_SECRET_ACCESS_KEY must be set.").to_string()
});

pub struct AuthenticationState(bool);

impl Clone for AuthenticationState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug)]
pub struct PortalAuthenticated(pub bool);

// When attaching this to service's function args
// Like pub async fn create_user(_: HttpRequest,authenticate:PortalAuthenticated)
impl FromRequest for PortalAuthenticated {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let value = req.extensions().get::<AuthenticationState>().cloned();
        let err_msg = "Request not authorized. Must interact with API's via web-browser";
        let result = match value {
            Some(_) => Ok(PortalAuthenticated(true)),
            None => Err(actix_web::error::ErrorForbidden(err_msg)),
        };
        ready(result)
    }
}


pub fn valid_incoming_source_checker(req: &ServiceRequest){

    let headers = req.headers();
    let access_key_header = headers.get("pioki-access-key");

    // only attach the value if found
    if let Some(incoming_access_key) = access_key_header {
        let access_key: &str = unsafe { LAZY_PIOKI_ACCESS_KEY.get(..).unwrap() }; // did this cuz I am lazy
        if incoming_access_key != access_key{return}
        // do some verification here
        let auth_data = AuthenticationState(true);
        req.extensions_mut().insert::<AuthenticationState>(auth_data);
    }
}