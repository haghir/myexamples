use actix_web::{
    http::{header},
    web, get, post, error,
    App, HttpServer, HttpResponse, Error
};
use actix_files::Files;
use std::{sync::{Arc, Mutex}, collections::HashMap};
use tera::{Tera, Context};
use log::info;

#[get("/")]
pub async fn index(
    templates_: web::Data<Arc<Mutex<Tera>>>
) -> Result<HttpResponse, Error> {
    info!("index()");

    let templates = templates_.lock().unwrap();
    let ctx = Context::new();

    let view = templates
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let templates = Arc::new(Mutex::new(Tera::new("templates/**/*").unwrap()));

    HttpServer::new(move || {
        App::new()
            .data(templates.clone())
            .service(Files::new("/js", "js").prefer_utf8(true))
            .service(index)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
