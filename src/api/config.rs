use actix_web::{web,HttpResponse,Responder};
use serde::Deserialize;

#[derive(Deserialize,Debug)]
struct User {
    username: String,
    password: String
}

async fn login(info: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome {} and your password is {}", info.username, info.password))
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