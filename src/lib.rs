use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run();
    // No .await here!
    Ok(server)
}
