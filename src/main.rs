use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet_controller(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet_controller))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
