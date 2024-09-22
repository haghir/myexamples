use bytes::Bytes;
use http_body_util::Full;
use hyper::{
    body::Incoming, header::LOCATION, server::conn::http1, service::Service, Method, Request,
    Response, StatusCode,
};
use hyper_util::rt::TokioIo;
use log::error;
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::{Arc, Mutex, RwLock},
};
use tera::{Context, Tera};
use tokio::net::TcpListener;

type DynError = Box<dyn std::error::Error + Send + Sync>;

// ===================================================================
// Data
// ===================================================================

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Record {
    id: u64,
    name: String,
    age: u8,
    dropped: bool,
}

// ===================================================================
// Pages
// ===================================================================

async fn index(
    tera: &Arc<Mutex<Tera>>,
    table: &Arc<RwLock<Vec<Record>>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let mut ctx = Context::new();
    let tera = match tera.lock() {
        Ok(value) => value,
        Err(_) => return Err("Something went wrong".into()),
    };
    let table = match table.read() {
        Ok(value) => value.to_vec(),
        Err(_) => return Err("Something went wrong".into()),
    };

    ctx.insert("table", &table);
    let view = tera.render("index.html", &ctx)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(view.into_bytes().into())?)
}

async fn new(
    tera: &Arc<Mutex<Tera>>,
    table: &Arc<RwLock<Vec<Record>>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let mut ctx = Context::new();
    let tera = match tera.lock() {
        Ok(value) => value,
        Err(_) => return Err("Something went wrong".into()),
    };
    let table = match table.read() {
        Ok(value) => value.to_vec(),
        Err(_) => return Err("Something went wrong".into()),
    };

    ctx.insert("table", &table);
    let view = tera.render("index.html", &ctx)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(view.into_bytes().into())?)
}

async fn create(
    req: Request<Incoming>,
    table: &Arc<RwLock<Vec<Record>>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let table = match table.write() {
        Ok(value) => value,
        Err(_) => return Err("Something went wrong".into()),
    };

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
    let mut ctx = Context::new();
    let tera = match tera.lock() {
        Ok(value) => value,
        Err(_) => return Err("Something went wrong".into()),
    };
    let table = match table.read() {
        Ok(value) => value.to_vec(),
        Err(_) => return Err("Something went wrong".into()),
    };

    ctx.insert("table", &table);
    let view = tera.render("edit.html", &ctx)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(view.into_bytes().into())?)
}

async fn update(
    req: Request<Incoming>,
    table: &Arc<RwLock<Vec<Record>>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let table = match table.write() {
        Ok(value) => value,
        Err(_) => return Err("Something went wrong".into()),
    };

    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, "/")
        .body("".into())?)
}

async fn drop(
    req: Request<Incoming>,
    tera: &Arc<Mutex<Tera>>,
    table: &Arc<RwLock<Vec<Record>>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let mut ctx = Context::new();
    let tera = match tera.lock() {
        Ok(value) => value,
        Err(_) => return Err("Something went wrong".into()),
    };
    let table = match table.read() {
        Ok(value) => value.to_vec(),
        Err(_) => return Err("Something went wrong".into()),
    };

    ctx.insert("table", &table);
    let view = tera.render("drop.html", &ctx)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(view.into_bytes().into())?)
}

async fn delete(
    req: Request<Incoming>,
    table: &Arc<RwLock<Vec<Record>>>,
) -> Result<Response<Full<Bytes>>, DynError> {
    let table = match table.write() {
        Ok(value) => value,
        Err(_) => return Err("Something went wrong".into()),
    };

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
    let mut ctx = Context::new();
    let tera = match tera.lock() {
        Ok(value) => value,
        Err(_) => return Err("Something went wrong".into()),
    };
    let table = match table.read() {
        Ok(value) => value.to_vec(),
        Err(_) => return Err("Something went wrong".into()),
    };

    ctx.insert("table", &table);
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
                (&Method::GET, "/new") => new(&tera, &table).await,
                (&Method::POST, "/create") => create(req, &table).await,
                (&Method::GET, "/edit") => edit(req, &tera, &table).await,
                (&Method::POST, "/update") => update(req, &table).await,
                (&Method::GET, "/drop") => drop(req, &tera, &table).await,
                (&Method::DELETE, "/delete") => delete(req, &table).await,
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
