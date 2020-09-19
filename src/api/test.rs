use actix_web::{web,Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Measurement {
    temperature: f32,
}

async fn get_temperature() -> impl Responder {
    web::Json(Measurement{ temperature: 37.1 })
}

pub fn test_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/json")
            .route(web::get().to(get_temperature))
    );
}