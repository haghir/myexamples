use actix_web::{get, App, HttpServer, HttpResponse, Result};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use log::info;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    info!("/ reached");
    Ok(HttpResponse::Ok().content_type("text/plain").body("index()"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(move || {
        App::new()
            .service(index)
    })
        .bind_openssl("0.0.0.0:3000", builder)?
        .run()
        .await
}
