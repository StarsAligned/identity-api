use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder}; 
use argon2::{self, Config}; 

#[derive(Debug, Serialize, Deserialize)] 
struct Info { email: String, password: String, }

#[derive(Debug, Serialize, Deserialize)] 
struct Claims { sub: String, }

#[post("/login")] pub async fn login(info: web::Json) -> impl Responder { 
    // This is a placeholder for the actual password retrieval and comparison 
    // You should replace this with actual code to retrieve the hashed password let user_password = "hashed_password_from_db";
    let config = Config::default();
    if argon2::verify_encoded(user_password, info.password.as_bytes(), &config).unwrap() {
        let claims = Claims { sub: info.email.clone() };
    }
}
