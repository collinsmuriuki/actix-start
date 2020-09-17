mod api;

use actix_web::{get, HttpServer, App, HttpResponse, web};
use actix_web::middleware::Logger;
use std::sync::Mutex;
use api::{config,scoped_config};


// This struct represents state
struct AppState {
    app_name: String,
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
}

#[get("/counter")]
async fn state_change(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {}", counter) // <- response with count
}

#[get("/show")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

#[get("/show/{id}")]
async fn user_detail(path: web::Path<(u32,)>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(AppState {
                app_name: String::from("This is a state value"),
                counter: Mutex::new(0),
            })
            .configure(config)
            .service(
                web::scope("/api")
                    .configure(scoped_config)
            )
            .service(state_change)
            .service(
                web::scope("/users")
                    .service(index)
                    .service(show_users)
                    .service(user_detail),
        ).default_service(web::route().to(|| HttpResponse::MethodNotAllowed()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
