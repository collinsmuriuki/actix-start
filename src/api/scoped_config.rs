use actix_web::{web,HttpResponse};

// this function could be located in a different module
pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("test")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}