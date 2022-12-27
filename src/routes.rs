use actix_web::{get, web, Responder};

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/announcements")]
async fn announcements() -> impl Responder {
    format!("Stub")
}

#[get("/rss")]
async fn rss() -> impl Responder {
    format!("Stub")
}

#[get("/active")]
async fn active() -> impl Responder {
    format!("Stub")
}
