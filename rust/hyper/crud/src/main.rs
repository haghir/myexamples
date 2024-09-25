use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::{
    body::Incoming, header::LOCATION, server::conn::http1, service::Service, Method, Request,
    Response, StatusCode,
};
use hyper_util::rt::TokioIo;
use log::error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, future::Future, net::SocketAddr, pin::Pin, sync::Arc};
use tera::{Context, Tera};
use tokio::{
    net::TcpListener,
    sync::{Mutex, RwLock},
};

type DynError = Box<dyn std::error::Error + Send + Sync>;

// ===================================================================
// Data
// ===================================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Record {
    id: usize,
    name: String,
    age: u8,
}

// ===================================================================
// Pages
// ===================================================================

async fn index(
    tera: &Arc<Mutex<Tera>>,
    table: &Arc<RwLock<Vec<Record>>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let tera = tera.lock().await;
    let table = table.read().await;
    let table_ref: &Vec<Record> = table.as_ref();

    let mut ctx = Context::new();
    ctx.insert("table", table_ref);
    let view = tera.render("index.html", &ctx)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(view.into_bytes().into())?)
}

async fn new(tera: &Arc<Mutex<Tera>>) -> Result<Response<Full<Bytes>>, DynError> {
    let ctx = Context::new();
    let tera = tera.lock().await;

    let view = tera.render("new.html", &ctx)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(view.into_bytes().into())?)
}

async fn create(
    req: Request<Incoming>,
    table: &Arc<RwLock<Vec<Record>>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let mut table = table.write().await;

    let bytes = req.collect().await?.to_bytes();
    let params: HashMap<String, String> = form_urlencoded::parse(&bytes).into_owned().collect();
    let id = table.len();

    table.push(Record {
        id,
        name: params.get("name").unwrap().clone(),
        age: params.get("age").unwrap().parse().unwrap(),
    });

    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, "/")
        .body("".into())?)
}

async fn edit(
    req: Request<Incoming>,
    tera: &Arc<Mutex<Tera>>,
    table: &Arc<RwLock<Vec<Record>>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let tera = tera.lock().await;
    let table = table.read().await;

    let query = req.uri().query().unwrap();
    let params: HashMap<String, String> = form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect();
    let id: usize = params.get("id").unwrap().parse().unwrap();
    let record = table.get(id).unwrap();

    let mut ctx = Context::new();
    ctx.insert("record", &record);
    let view = tera.render("edit.html", &ctx)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(view.into_bytes().into())?)
}

async fn update(
    req: Request<Incoming>,
    table: &Arc<RwLock<Vec<Record>>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let mut table = table.write().await;

    let bytes = req.collect().await?.to_bytes();
    let params: HashMap<String, String> = form_urlencoded::parse(&bytes).into_owned().collect();
    let id: usize = params.get("id").unwrap().parse().unwrap();

    let record = table.get_mut(id).unwrap();
    record.name = params.get("name").unwrap().clone();
    record.age = params.get("age").unwrap().parse().unwrap();

    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, "/")
        .body("".into())?)
}

async fn show(
    req: Request<Incoming>,
    tera: &Arc<Mutex<Tera>>,
    table: &Arc<RwLock<Vec<Record>>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let tera = tera.lock().await;
    let table = table.read().await;

    let query = req.uri().query().unwrap();
    let params: HashMap<String, String> = form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect();
    let id: usize = params.get("id").unwrap().parse().unwrap();
    let record = table.get(id).unwrap();

    let mut ctx = Context::new();
    ctx.insert("record", &record);
    let view = tera.render("show.html", &ctx)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(view.into_bytes().into())?)
}

// ===================================================================
// Service
// ===================================================================

#[derive(Debug, Clone)]
struct MyService {
    tera: Arc<Mutex<Tera>>,
    table: Arc<RwLock<Vec<Record>>>,
}

impl Service<Request<Incoming>> for MyService {
    type Response = Response<Full<Bytes>>;
    type Error = DynError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let tera = Arc::clone(&self.tera);
        let table = Arc::clone(&self.table);

        Box::pin(async move {
            match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => index(&tera, &table).await,
                (&Method::GET, "/new") => new(&tera).await,
                (&Method::POST, "/create") => create(req, &table).await,
                (&Method::GET, "/edit") => edit(req, &tera, &table).await,
                (&Method::POST, "/update") => update(req, &table).await,
                (&Method::GET, "/show") => show(req, &tera, &table).await,
                _ => Err("Something went wrong".into()),
            }
        })
    }
}

// ===================================================================
// Entry Point
// ===================================================================

#[tokio::main]
async fn main() -> Result<(), DynError> {
    env_logger::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    let tera = Arc::new(Mutex::new(Tera::new("html/**/*")?));
    let table = Arc::new(RwLock::new(Vec::new()));
    let svc = MyService { tera, table };

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let svc_clone = svc.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, svc_clone).await {
                error!("{:?}", err);
            }
        });
    }
}
