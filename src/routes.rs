use crate::announcement::Announcement;
use crate::recurring::Recurring;
use actix_web::{get, post, web, Responder};
use serde::Deserialize;
use std::sync::Arc;

#[get("/recurring/list")]
async fn recurring_list(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
) -> impl Responder {
    let mut ret = String::new();

    let mut count = 0;
    loop {
        match pool.get().unwrap().query_row(
            "select * from recurring limit 1 offset ?",
            [count],
            |row| {
                let id = row.get::<usize, String>(0).unwrap();
                let title = row.get::<usize, String>(1).unwrap();
                let body = row.get::<usize, String>(2).unwrap();
                let created = row.get::<usize, String>(3).unwrap();
                let mode = row.get::<usize, String>(4).unwrap();
                let time_frame = row.get::<usize, String>(5).unwrap();

                ret += format!(
                    "<div class='recurring'>
        <div class='date'>
          <div>Created: {created}</div>
        </div>
        <div class='title' onclick='update_recurring(\"{id}\", \"title\", \"{title}\")'>{title}</div>
        <div class='body' onclick='update_recurring(\"{id}\", \"body\", \"{body}\")'>{body}</div>
        <div class='when' onclick='update_recurring(\"{id}\", \"time_frame\", \"{time_frame}\")'>{mode} * Happens Every Tuesday, triggered next at Sat Dec 31 12:56:52 AM CST 2022</div>
        <button style='color:Red' onclick='recur_daily(\"{id}\")'>Daily</button>
        <button style='color:Orange' onclick='recur_weekly(\"{id}\")'>Weekly</button>
        <button style='color:#770' onclick='recur_monthly(\"{id}\")'>Monthly</button>
        <button style='color:Green' onclick='recur_yearly(\"{id}\")'>Yearly</button>
        <button style='color:maroon' onclick='delete_recur(\"{id}\")'>Delete</button>
        <div class='id'>{id}</div>
      </div>")
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

#[get("/announcements/list/{publish_status}")]
async fn announcements_list(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
    publish_status: web::Path<String>,
) -> impl Responder {
    let mut ret = String::new();

    let mut count = 0;
    loop {
        match pool.get().unwrap().query_row(
            "select * from announcements limit 1 offset ?",
            [count],
            |row| {
                let id = row.get::<usize, String>(5).unwrap();
                let status = row.get::<usize, String>(0).unwrap();

                if publish_status.to_string() == "published" && status != "published" {
                    return Ok(());
                }

                if publish_status.to_string() == "unpublished" && status == "published" {
                    return Ok(());
                }

                ret += format!(
                    "<div class='announcement-{}'>
        <div class='date'>
          <div>Created: {}</div>
          <div>Expires: {}</div>
          <div>Scheduled: {}</div>
        </div>
        <div class='title' onclick='update_announcement(\"{id}\", \"title\", \"{}\")'>{}</div>
        <div class='body' onclick='update_announcement(\"{id}\", \"body\", \"{}\")'>{}</div>
        <button style='color:green' onclick='set_state(\"{id}\", \"approved\")'>Approve</button>
        <button style='color:#770' onclick='update_schedule(\"{id}\")'>Schedule</button>
        <button style='color:orange' onclick='update_expiration(\"{id}\")'>Set Expiration</button>
        <button style='color:red' onclick='set_state(\"{id}\", \"denied\")'>Deny</button>
        <button style='color:maroon' onclick='delete_announcement(\"{id}\")'>Delete</button>
        <button style='color:blue'>Hide</button>
        <button style='color:purple' onclick='set_state(\"{id}\", \"published\")'>Publish</button>
        <div class='id'>{id}</div>
      </div>",
                    row.get::<usize, String>(0).unwrap(), // status
                    row.get::<usize, String>(1).unwrap(), // created
                    row.get::<usize, String>(6).unwrap(), // expires
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

#[get("/announcements/json/")]
async fn announcements_json(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
) -> impl Responder {
    let mut ret: Vec<Announcement> = Vec::new();

    let mut count = 0;
    loop {
        match pool.get().unwrap().query_row(
            "select * from announcements limit 1 offset ?",
            [count],
            |row| {
                let announcement = Announcement::new(
                    row.get::<usize, String>(3).unwrap(), // title
                    row.get::<usize, String>(4).unwrap(), // body
                    row.get::<usize, String>(1).unwrap(), // created
                    row.get::<usize, String>(2).unwrap(), // scheduled
                    row.get::<usize, String>(5).unwrap(), // id
                    row.get::<usize, String>(0).unwrap(), // status
                    row.get::<usize, String>(6).unwrap(), // expires
                );

                ret.push(announcement);

                Ok(())
            },
        ) {
            Ok(_) => {}
            Err(_) => break,
        }

        count += 1;

        ()
    }

    web::Json(ret)
}

#[post("/recurring/add")]
async fn recurring_add(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
    recurring: web::Json<Recurring>,
) -> impl Responder {
    pool.get()
        .unwrap()
        .execute(
            "insert into recurring values (
                ?,
                ?,
                ?,
                ?,
                ?,
                ?
                );",
            [
                &recurring.id,
                &recurring.title,
                &recurring.body,
                &recurring.created,
                &recurring.mode,
                &recurring.time_frame,
            ],
        )
        .unwrap();

    "Add successful"
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
                &announcement.expires,
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

#[post("/recurring/delete/{id}")]
async fn recurring_delete(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
    id: web::Path<String>,
) -> impl Responder {
    pool.get()
        .unwrap()
        .execute("delete from recurring where id=?", [id.as_str()])
        .unwrap();

    "Delete successful"
}

#[derive(Deserialize)]
struct Change {
    field: String,
    content: String,
}

#[post("/announcements/update/{id}")]
async fn announcements_update(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
    id: web::Path<String>,
    change: web::Json<Change>,
) -> impl Responder {
    let sql = format!(
        "update announcements set {} = \"{}\" where id = \"{}\";",
        change.field,
        change.content,
        id.to_string()
    );

    pool.get().unwrap().execute(sql.as_str(), []).unwrap();

    "Change successful"
}

#[get("/rss")]
async fn rss() -> impl Responder {
    format!("Stub")
}

#[get("/active")]
async fn active() -> impl Responder {
    format!("Stub")
}
