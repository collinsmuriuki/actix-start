use actix_web::{get,HttpServer,App,HttpResponse,web};


// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name

    format!("Hello {}!", app_name) // <- response with app_name
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
            .data(AppState {
                app_name: String::from("This is a state value"),
            })
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
