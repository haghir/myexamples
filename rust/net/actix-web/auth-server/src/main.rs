use actix_web::{get, App, HttpServer, HttpResponse, Result};
use actix_web_httpauth::extractors::basic::BasicAuth;
use log::debug;
use env_logger as logger;

fn verify(user_id: &String, password: &str) -> bool {
    if user_id == "foo" {
        password == "foofoo"
    } else if user_id == "bar" {
        password == "barbar"
    } else if user_id == "baz" {
        password == "bazbaz"
    } else {
        false
    }
}

#[get("/auth")]
async fn auth(auth: BasicAuth) -> Result<HttpResponse> {
    let user_id = format!("{}", auth.user_id());
    debug!("user_id: {}", user_id);

    if let Some(password) = auth.password() {
        debug!("password: {}", password);
        if verify(&user_id, &password) {
            Ok(HttpResponse::Ok().finish())
        } else {
            Ok(HttpResponse::Forbidden().finish())
        }
    } else {
        debug!("password is empty");
        Ok(HttpResponse::Forbidden().finish())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init();

    HttpServer::new(move || {
        App::new()
            .service(auth)
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
