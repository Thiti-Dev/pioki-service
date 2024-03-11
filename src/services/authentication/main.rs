use google_oauth::AsyncClient;
use serde::Serialize;
use actix_web::{HttpRequest, HttpResponse, Responder};


#[derive(Serialize)] // Required for serde
struct ProviderPayload {
  email: String,
  id: String,
}
  
#[warn(dead_code)]
pub async fn get_oauth_crendential(_body: String,req: HttpRequest) -> impl Responder {
    let headers = req.headers();
    let auth_header = headers.get("Authorization");
    
    match auth_header{
        Some(token) => {
            let access_token = token.to_str().unwrap();

            // token verification phase
            let client = AsyncClient::new("");

            let payload = client.validate_access_token(access_token).await;
            if let Ok(g_payload) = payload{
                let data: ProviderPayload = ProviderPayload {email: g_payload.email.unwrap().to_string(),id: g_payload.sub};
                return HttpResponse::Ok().json(data)
            }

            HttpResponse::Unauthorized().body("Invalid given authorization header")
        },
        None => {
            HttpResponse::Unauthorized().body("No authorization header found")
        },
    }
}