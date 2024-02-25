use actix_web::{
    get, post, error,
    web::Data, web::block,
    App, HttpServer, HttpResponse, Error
};
use actix_multipart::Multipart;
use futures_util::TryStreamExt;
use std::sync::Mutex;
use std::fs::{File, create_dir_all};
use std::io::Write;
use tera::{Tera, Context};
use sha2::{Sha256, Digest};
use env_logger as logger;

#[get("/")]
pub async fn index(
    templates_: Data<Mutex<Tera>>
) -> Result<HttpResponse, Error> {
    let names: Vec<String> = vec![];
    let hashes: Vec<String> = vec![];

    let mut ctx = Context::new();
    ctx.insert("names", &names);
    ctx.insert("hashes", &hashes);

    let templates = templates_.lock().unwrap();
    let view = templates
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[post("/")]
pub async fn upload(
    mut payload: Multipart,
    templates_: Data<Mutex<Tera>>
) -> Result<HttpResponse, Error> {

    let mut names: Vec<String> = vec![];
    let mut hashes: Vec<String> = vec![];
    while let Some(mut field) = payload.try_next().await? {
        // retrieves the file name
        let content_disposition = field.content_disposition();
        let filename = content_disposition
            .get_filename()
            .map_or_else(|| "", |v| v);
        names.push(String::from(filename));

        // saves the file, and calculates a sha256 of it
        block(|| create_dir_all("tmp")).await??;
        let path = format!("tmp/{}", filename);
        let mut f = block(|| File::create(path)).await??;
        let mut hasher = Sha256::new();
        while let Some(chunk) = field.try_next().await? {
            hasher.update(&chunk);
            f = block(move || f.write_all(&chunk).map(|_| f)).await??;
        }
        let hash = hex::encode(hasher.finalize());
        hashes.push(hash);
    }

    let mut ctx = Context::new();
    ctx.insert("names", &names);
    ctx.insert("hashes", &hashes);

    let templates = templates_.lock().unwrap();
    let view = templates
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init();

    let templates = Data::new(Mutex::new(Tera::new("html/**/*").unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&templates))
            .service(index)
            .service(upload)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
