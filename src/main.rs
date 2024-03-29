mod login;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(login::login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}