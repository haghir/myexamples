use actix_files as fs;
use actix_web::{
    get,
    http::header::{ContentDisposition, DispositionType},
    post,
    web::Json,
    App, HttpRequest, HttpServer, Responder, Result,
};
use common::{Request, Response};
use env_logger as logger;
use log::debug;
use std::path::PathBuf;

async fn get_resource(path: &str) -> Result<fs::NamedFile> {
    debug!("request: {}", path);

    let mut path_buf = PathBuf::new();
    path_buf.push("frontend/dist");
    path_buf.push(path);

    debug!("returns: {}", path_buf.display());

    let file = fs::NamedFile::open(path_buf)?;

    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

#[get("/")]
async fn index() -> Result<fs::NamedFile> {
    get_resource("index.html").await
}

#[get("/{path:.*\\.(html|js|wasm)}")]
async fn resource(req: HttpRequest) -> Result<fs::NamedFile> {
    let path: PathBuf = req.match_info().query("path").parse().unwrap();

    get_resource(path.to_str().unwrap()).await
}

#[post("/auth")]
async fn auth(req: Json<Request>) -> Result<impl Responder> {
    debug!("auth PIN: {}", req.pin);

    Ok(Json(match req.pin.as_str() {
        "1234" => Response::new("Alice", 20),
        "2345" => Response::new("Bobby", 30),
        "3456" => Response::new("Crith", 40),
        _ => Response::err("Invalid PIN"),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init();

    HttpServer::new(move || App::new().service(index).service(resource).service(auth))
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
