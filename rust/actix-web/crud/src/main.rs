use actix_web::{
    http::{header}, web::{Data, Path, Form},
    get, post, error,
    App, HttpServer, HttpResponse, Error
};
use std::{sync::Mutex, collections::HashMap};
use serde::{Serialize, Deserialize};
use tera::{Tera, Context};
use log::info;

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    id: i64,
    text: String,
}

impl Record {
    fn new(id: i64, text: &str) -> Record {
        Record {
            id,
            text: String::from(text),
        }
    }
}

pub struct Storage {
    table: HashMap<i64, Record>,
    next: i64,
}

impl Storage {
    fn new () -> Storage {
        Storage {
            table: HashMap::new(),
            next: 0,
        }
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        info!("The instance of Storage will be dropped.");
    }
}

#[derive(Deserialize)]
pub struct PathParams {
    id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct NewForm {
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditForm {
    id: i64,
    text: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteForm {
    id: i64,
}

#[get("/")]
pub async fn index(
    templates_: Data<Mutex<Tera>>,
    storage_: Data<Mutex<Storage>>
) -> Result<HttpResponse, Error> {
    let storage = storage_.lock().unwrap();
    let templates = templates_.lock().unwrap();
    let mut ctx = Context::new();
    let table: Vec<&Record> = storage.table.values().collect();

    ctx.insert("table", &table);

    let view = templates
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[get("/show/{id}")]
pub async fn show(
    path: Path<PathParams>,
    templates_: Data<Mutex<Tera>>,
    storage_: Data<Mutex<Storage>>
) -> Result<HttpResponse, Error> {
    let storage = storage_.lock().unwrap();
    let templates = templates_.lock().unwrap();
    let mut ctx = Context::new();

    ctx.insert("record", &storage.table[&path.id]);

    let view = templates
        .render("show.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[get("/new")]
pub async fn new(
    templates_: Data<Mutex<Tera>>
) -> Result<HttpResponse, Error> {
    let templates = templates_.lock().unwrap();
    let ctx = Context::new();

    let view = templates
        .render("new.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[post("/new")]
pub async fn do_new(
    form: Form<NewForm>,
    storage_: Data<Mutex<Storage>>
) -> Result<HttpResponse, Error> {
    let mut storage = storage_.lock().unwrap();

    let id = storage.next;
    storage.next += 1;

    let record = Record::new(id, form.text.as_str());
    storage.table.insert(id, record);

    Ok(HttpResponse::SeeOther().append_header((header::LOCATION, "/")).finish())
}

#[get("/edit/{id}")]
pub async fn edit(
    path: Path<PathParams>,
    templates_: Data<Mutex<Tera>>,
    storage_: Data<Mutex<Storage>>
) -> Result<HttpResponse, Error> {
    let storage = storage_.lock().unwrap();
    let templates = templates_.lock().unwrap();
    let mut ctx = Context::new();

    ctx.insert("record", &storage.table[&path.id]);

    let view = templates
        .render("edit.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[post("/edit")]
pub async fn do_edit(
    form: Form<EditForm>,
    storage_: Data<Mutex<Storage>>
) -> Result<HttpResponse, Error> {
    let mut storage = storage_.lock().unwrap();

    if let Some(record) = storage.table.get_mut(&form.id) {
        record.text = form.text.clone();
    }

    Ok(HttpResponse::SeeOther().append_header((header::LOCATION, "/")).finish())
}

#[get("/delete/{id}")]
pub async fn delete(
    path: Path<PathParams>,
    templates_: Data<Mutex<Tera>>,
    storage_: Data<Mutex<Storage>>
) -> Result<HttpResponse, Error> {
    let storage = storage_.lock().unwrap();
    let templates = templates_.lock().unwrap();
    let mut ctx = Context::new();

    ctx.insert("record", &storage.table[&path.id]);

    let view = templates
        .render("delete.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[post("/delete")]
pub async fn do_delete(
    form: Form<DeleteForm>,
    storage_: Data<Mutex<Storage>>
) -> Result<HttpResponse, Error> {
    let mut storage = storage_.lock().unwrap();

    storage.table.remove(&form.id);

    Ok(HttpResponse::SeeOther().append_header((header::LOCATION, "/")).finish())
}

#[get("/panic")]
pub async fn panic() -> Result<HttpResponse, Error> {
    panic!("/panic was called");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let templates = Data::new(Mutex::new(Tera::new("templates/**/*").unwrap()));
    let storage = Data::new(Mutex::new(Storage::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&storage))
            .app_data(Data::clone(&templates))
            .service(index)
            .service(show)
            .service(new)
            .service(do_new)
            .service(edit)
            .service(do_edit)
            .service(delete)
            .service(do_delete)
            .service(panic)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
