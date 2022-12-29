use crate::announcement::Announcement;
use actix_web::{get, post, web, Responder};
use serde::Deserialize;
use std::sync::Arc;

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/announcements/list")]
async fn announcements_list(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
) -> impl Responder {
    let mut ret = String::new();

    let mut count = 0;
    loop {
        match pool.get().unwrap().query_row(
            "select * from announcements limit 1 offset ?",
            [count],
            |row| {
                let id = row.get::<usize, String>(5).unwrap();

                ret += format!(
                    "<div class='announcement-{}'>
        <div class='date'>
          <div>Created: {}</div>
          <div>Scheduled: {}</div>
        </div>
        <div class='title' onclick='update_announcement(\"{id}\", \"title\", \"{}\")'>{}</div>
        <div class='body' onclick='update_announcement(\"{id}\", \"body\", \"{}\")'>{}</div>
        <button style='color:green' onclick='set_state(\"{id}\", \"approved\")'>Approve</button>
        <button style='color:#770'>Schedule</button>
        <button style='color:red' onclick='set_state(\"{id}\", \"denied\")'>Deny</button>
        <button style='color:maroon' onclick='delete_announcement(\"{id}\")'>Delete</button>
        <button style='color:blue'>Hide</button>
        <div class='id'>{id}</div>
      </div>",
                    row.get::<usize, String>(0).unwrap(), // status
                    row.get::<usize, String>(1).unwrap(), // created
                    row.get::<usize, String>(2).unwrap(), // scheduled
                    row.get::<usize, String>(3).unwrap(), // title
                    row.get::<usize, String>(3).unwrap(), // title
                    row.get::<usize, String>(4).unwrap(), // body
                    row.get::<usize, String>(4).unwrap(), // body
                )
                .as_str();

                Ok(())
            },
        ) {
            Ok(_) => {}
            Err(_) => break,
        }

        count += 1;

        ()
    }
    ret
}

#[post("/announcements/add")]
async fn announcements_add(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
    announcement: web::Json<Announcement>,
) -> impl Responder {
    pool.get()
        .unwrap()
        .execute(
            "insert into announcements values (
                ?,
                ?,
                ?,
                ?,
                ?,
                ?
                );",
            [
                &announcement.status,
                &announcement.created,
                &announcement.scheduled,
                &announcement.title,
                &announcement.body,
                &announcement.id,
            ],
        )
        .unwrap();

    "Add successful"
}

#[post("/announcements/delete/{id}")]
async fn announcements_delete(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
    id: web::Path<String>,
) -> impl Responder {
    pool.get()
        .unwrap()
        .execute("delete from announcements where id=?", [id.as_str()])
        .unwrap();

    "Delete successful"
}

    let mut announcements = Vec::new();

    announcements.push(Announcement::new(
        "title",
        "body",
        "created",
        "scheduled",
        "id",
        "approved",
    ));

    for announcement in announcements {
        ret += format!(
            "<div class='announcement-{}'>
    <div class='date'>
      <div>Created: {}</div>
      <div>Scheduled: {}</div>
    </div>
    <div class='title'>{}</div>
    <div class='body'>{}</div>
    <button style='color:green'>Approve</button>
    <button style='color:#770'>Schedule</button>
    <button style='color:red'>Deny</button>
    <div class='id'>{}</div>
  </div>",
            announcement.status,
            announcement.created,
            announcement.scheduled,
            announcement.title,
            announcement.body,
            announcement.id,
        )
        .as_str();
    }

    ret
}

#[get("/rss")]
async fn rss() -> impl Responder {
    format!("Stub")
}

#[get("/active")]
async fn active() -> impl Responder {
    format!("Stub")
}
