use actix_web::{web,HttpResponse,HttpRequest,Responder,Error};
use serde::{Deserialize,Serialize};
use serde_json::to_string;
use futures::future::{ready, Ready};

#[derive(Deserialize,Serialize,Debug)]
struct User {
    username: String,
    password: String
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse,Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
        ))
    }
}

async fn login(info: web::Json<User>) -> impl Responder {
    let username = info.username.clone();
    let password = info.password.clone();
    User { username, password }
}

// this function could be located in a different module
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/config")
            .route(web::get().to(|| HttpResponse::Ok().body("config")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
            .route(web::post().to(login)),
    );
}