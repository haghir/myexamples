use std::{sync::{Arc, Mutex}};
use log::debug;
use env_logger as logger;
use actix_files as fs;
use actix_web::{web, web::Data, get, error};
use actix_web::{ App, HttpServer, HttpRequest, HttpResponse, Error};
use actix_web::http::header::{ContentDisposition, DispositionType};
use tera::{Tera, Context};

#[get("/")]
pub async fn index(
    templates_: web::Data<Arc<Mutex<Tera>>>
) -> Result<HttpResponse, Error> {
    debug!("index.html was requested.");

    let templates = templates_.lock().unwrap();
    let ctx = Context::new();

    let view = templates
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[get("/hello")]
pub async fn hello() -> Result<HttpResponse, Error> {
    debug!("/hello");

    Ok(HttpResponse::Ok().content_type("text/plain").body("Hello world!"))
}

#[get("/{path:js/.*\\.js}")]
pub async fn javascript(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let path: std::path::PathBuf = req.match_info()
        .query("path").parse().unwrap();

    debug!("{:?} was requested.", path);

    let file = fs::NamedFile::open(path)?;

    Ok(file
       .use_last_modified(true)
       .set_content_disposition(ContentDisposition {
           disposition: DispositionType::Inline,
           parameters: vec![],
       }))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    logger::init();

    let templates = Arc::new(Mutex::new(Tera::new("templates/**/*").unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(templates.clone()))
            .service(index)
            .service(javascript)
            .service(hello)
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
