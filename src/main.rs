use actix_files::Files;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;

mod announcement;
mod recurring;
mod routes;

static SESSION_SIGNING_KEY: &[u8] = &[0; 64]; // Just an example

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sqlite_file = "default.db";
    let sqlite_connection_manager = SqliteConnectionManager::file(sqlite_file);

    let sqlite_pool = r2d2::Pool::new(sqlite_connection_manager)
        .expect("Failed to create SQLite connection from file.");

    let query = "
    create table if not exists announcements (
        status text,
        created text,
        scheduled text,
        title text,
        body text,
        id text,
        expires text,
        hidden text,
        tags text
    );

    create table if not exists recurring (
        id text,
        title text,
        body text,
        created text,
        mode text,
        time_frame text,
        hidden text,
        tags text
    );
    ";

    sqlite_pool.get().unwrap().execute_batch(query).unwrap();

    let pool_arc = Arc::new(sqlite_pool);

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let key = actix_web::cookie::Key::from(SESSION_SIGNING_KEY);

    let host = "0.0.0.0";
    let port = 8080;

    log::info!("Serving on {} port {}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool_arc.clone()))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(routes::active)
            .service(routes::announcements_add)
            .service(routes::announcements_delete)
            .service(routes::announcements_hide)
            .service(routes::announcements_json)
            .service(routes::announcements_list_published)
            .service(routes::announcements_list_unpublished)
            .service(routes::announcements_update)
            .service(routes::recurring_add)
            .service(routes::recurring_delete)
            .service(routes::recurring_hide)
            .service(routes::recurring_list)
            .service(routes::recurring_update)
            .service(routes::rss)
            .service(Files::new("/images", "static/images/").show_files_listing())
            .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind((host, port))?
    .run()
    .await
}
