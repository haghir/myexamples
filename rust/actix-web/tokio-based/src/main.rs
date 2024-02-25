use actix_web::{
    web, web::Data, get, error,
    App, HttpServer, HttpResponse, Error
};
use std::{sync::{Arc, Mutex}};
use tera::{Tera, Context};

#[get("/")]
pub async fn index(
    templates_: web::Data<Arc<Mutex<Tera>>>
) -> Result<HttpResponse, Error> {
    let templates = templates_.lock().unwrap();
    let ctx = Context::new();

    let view = templates
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let templates = Arc::new(Mutex::new(Tera::new("templates/**/*").unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(templates.clone()))
            .service(index)
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
