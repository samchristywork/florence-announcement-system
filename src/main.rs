use actix_files::Files;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;

mod announcement;
mod routes;

static SESSION_SIGNING_KEY: &[u8] = &[0; 64]; // Just an example

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = sqlite::open("data.sqlite").unwrap();

    let query = "
    create table if not exists users (name text, age integer);
    insert into users values ('Alice', 42);
    insert into users values ('Bob', 69);
    insert into users values ('Charlie', 69);
";

    connection.execute(query).unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let key = actix_web::cookie::Key::from(SESSION_SIGNING_KEY);

    let host = "0.0.0.0";
    let port = 8080;
    log::info!("Serving on {} port {}", host, port);
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(routes::hello)
            .service(routes::announcements)
            .service(routes::rss)
            .service(routes::active)
            .service(Files::new("/images", "static/images/").show_files_listing())
            .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
