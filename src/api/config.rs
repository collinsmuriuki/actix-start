use actix_web::{web,HttpResponse};

// this function could be located in a different module
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/config")
            .route(web::get().to(|| HttpResponse::Ok().body("config")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}