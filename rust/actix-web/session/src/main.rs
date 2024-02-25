use actix_web::{
    web::{Data, Form},
    get, post, error,
    App, HttpServer, HttpResponse, Result,
    middleware
};
use actix_session::{storage::RedisActorSessionStore, Session, SessionMiddleware};
use std::sync::Mutex;
use serde::Deserialize;
use tera::{Tera, Context};
use env_logger as logger;

#[derive(Deserialize)]
pub struct LoginForm {
    user_id: String,
    password: String,
}

fn authenticate(user_id: &str, password: &str) -> bool {
    if user_id.eq("foo") {
        password.eq("foofoo")
    } else if user_id.eq("bar") {
        password.eq("barbar")
    } else if user_id.eq("baz") {
        password.eq("bazbaz")
    } else {
        false
    }
}

fn get_user_id(session: &Session) -> Result<String> {
    if let Some(user_id) = session.get("user_id")? {
        Ok(user_id)
    } else {
        Ok(String::from(""))
    }
}

#[get("/")]
pub async fn index(
    session: Session,
    templates_: Data<Mutex<Tera>>
) -> Result<HttpResponse> {
    let user_id = get_user_id(&session)?;

    let templates = templates_.lock().unwrap();
    let mut ctx = Context::new();

    ctx.insert("user_id", &user_id);

    let view = templates
        .render("index.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

fn login_common(
    message_: Option<&str>,
    session: Session,
    templates_: Data<Mutex<Tera>>
) -> Result<HttpResponse> {
    let user_id = get_user_id(&session)?;

    let message: &str;
    let has_message: bool;
    if let Some(message_text) = message_ {
        message = message_text;
        has_message = true;
    } else {
        message = "";
        has_message = false;
    }

    let templates = templates_.lock().unwrap();
    let mut ctx = Context::new();

    ctx.insert("user_id", &user_id);
    ctx.insert("message", &message);
    ctx.insert("has_message", &has_message);

    let view = templates
        .render("login.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[get("/login")]
pub async fn login(
    session: Session,
    templates_: Data<Mutex<Tera>>
) -> Result<HttpResponse> {
    login_common(None, session, templates_)
}

#[post("/login")]
pub async fn do_login(
    form: Form<LoginForm>,
    session: Session,
    templates_: Data<Mutex<Tera>>
) -> Result<HttpResponse> {
    let user_id = form.user_id.as_str();
    let password = form.password.as_str();

    if !authenticate(user_id, password) {
        return login_common(Some("Failed to login."), session, templates_);
    }

    session.insert("user_id", String::from(user_id))?;

    // If the logging in was succeeded, redirect to the index.
    Ok(HttpResponse::SeeOther()
        .append_header(("LOCATION", "/"))
        .finish()
    )
}

#[get("/secret")]
pub async fn secret(
    session: Session,
    templates_: Data<Mutex<Tera>>
) -> Result<HttpResponse> {
    let user_id: String;
    if let Some(user_id_) = session.get("user_id")? {
        user_id = user_id_;
    } else {
        return Ok(HttpResponse::TemporaryRedirect()
            .append_header(("LOCATION", "/"))
            .finish()
        );
    }

    let templates = templates_.lock().unwrap();
    let mut ctx = Context::new();
    ctx.insert("user_id", &user_id);

    let view = templates
        .render("secret.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init();

    let private_key = actix_web::cookie::Key::generate();

    // Tera template engine
    let templates = Data::new(Mutex::new(Tera::new("html/**/*").unwrap()));

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::builder(
                RedisActorSessionStore::new("127.0.0.1:6379"),
                private_key.clone())
                .build()
            )
            .wrap(middleware::Logger::default())
            .app_data(Data::clone(&templates))
            .service(index)
            .service(login)
            .service(do_login)
            .service(secret)
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
