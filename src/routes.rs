use crate::announcement::Announcement;
use crate::recurring::Recurring;
use actix_web::{get, post, web, Responder};
use chrono::{DateTime, Duration};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug)]
struct Response {
    announcements: Vec<Announcement>,
    recurring: Vec<Recurring>,
}

fn get_next_time(created: &str, time_frame: &str) -> Result<String, i32> {
    let words = time_frame.split(" ").collect::<Vec<&str>>();

    let mut next = DateTime::parse_from_str(
        format!("{} -0600", created).as_str(),
        "%m/%d/%Y, %l:%M:%S %p CT %z",
    )
    .unwrap();

    while next < chrono::offset::Utc::now() {
        next = next + Duration::days(1);
    }

    let mut found = false;

    if words.get(0).unwrap() == &"Every" {
        for _ in 1..60 {
            if words
                .get(1)
                .unwrap()
                .eq(&next.format("%A").to_string().as_str())
            {
                found = true;
                break;
            }

            if words
                .get(1)
                .unwrap()
                .eq(&next.format("%e").to_string().as_str())
            {
                found = true;
                break;
            }

            next = next + Duration::days(1);
        }
    }

    let mut next = format!("{}", next.format("%-m/%-d/%Y, %l:%M:%S %p CT").to_string());

    if !found {
        next = String::new() + "Incalculable";
    }

    Ok(next)
}

#[get("/recurring/list")]
async fn recurring_list(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
) -> impl Responder {
    let mut ret = String::new();

    let mut count = 0;
    loop {
        match pool.get().unwrap().query_row(
            "select * from recurring order by hidden asc limit 1 offset ?",
            [count],
            |row| {
                let id = row.get::<usize, String>(0).unwrap();
                let title = row.get::<usize, String>(1).unwrap();
                let body = row.get::<usize, String>(2).unwrap();
                let created = row.get::<usize, String>(3).unwrap();
                let mode = row.get::<usize, String>(4).unwrap();
                let time_frame = row.get::<usize, String>(5).unwrap();
                let hide = row.get::<usize, String>(6).unwrap();
                let tags = row.get::<usize, String>(7).unwrap();

                let next =
                    match get_next_time(created.as_str(), time_frame.as_str()){
                        Ok(s) => s,
                        Err(_) => String::new () + "Incalculable",
                    };

                let view_class = match hide.as_str() {
                    "true" => "recur-hidden",
                    _ => "",
                };

                ret += format!(
                    "<div class='recurring {view_class}'>
        <div class='date'>
          <div>From: {created}</div>
          <div>Next: {next}</div>
        </div>
        <div style='display: grid; grid-template-columns: 1fr 1fr'>
          <div class='title' onclick='update_recurring(\"{id}\", \"title\", \"{title}\")'>{title}</div>
        </div>
        <div class='tags' onclick='update_recurring(\"{id}\", \"tags\", \"{tags}\"'>{tags}</div>
        <div class='body' onclick='update_recurring(\"{id}\", \"body\", \"{body}\")'>{body}</div>
        <div class='when' onclick='update_recurring(\"{id}\", \"time_frame\", \"{time_frame}\")'>Time Frame: {time_frame}</div>
        <button style='color:Green' onclick=''>Set Start Date</button>
        <button style='color:Orange' onclick=''>Set Expiration</button>
        <button style='color:maroon' onclick='delete_recur(\"{id}\")'>Delete</button>
        <button style='color:blue' onclick='hide_recurring(\"{id}\")'>Hide</button>
        <button style='color:blue' onclick='unhide_recurring(\"{id}\")' class='unhide'>Unhide</button>
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

#[get("/announcements/list/published")]
async fn announcements_list_published(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
) -> impl Responder {
    let mut ret = String::new();

    let mut count = 0;
    loop {
        match pool.get().unwrap().query_row(
            "select * from announcements where status='published' order by hidden asc limit 1 offset ?",
            [count],
            |row| {
                let id = row.get::<usize, String>(5).unwrap();

                let hide = row.get::<usize, String>(7).unwrap();
                let view_class = match hide.as_str() {
                    "true" => "announcement-hidden",
                    _ => "",
                };

                    ret += format!(
                        "<div class='announcement-{} {view_class}'>
        <div class='date'>
          <div>Created: {}</div>
          <div>Expires: {}</div>
          <div>Scheduled: {}</div>
        </div>
        <div style='display: grid; grid-template-columns: 1fr 1fr'>
          <div class='title' onclick='update_announcement(\"{id}\", \"title\", \"{}\")'>{}</div>
        </div>
        <div class='tags' onclick='update_recurring(\"{id}\", \"tags\", \"{}\"'>{}</div>
        <div class='body' onclick='update_announcement(\"{id}\", \"body\", \"{}\")'>{}</div>
        <button style='color:red' onclick='set_state(\"{id}\", \"neutral\")'>Un-Publish</button>
        <button style='color:maroon' onclick='delete_announcement(\"{id}\")'>Delete</button>
        <button style='color:blue' onclick='hide_announcement(\"{id}\")'>Hide</button>
        <button style='color:blue' onclick='unhide_announcement(\"{id}\")' class='unhide'>Unhide</button>
        <div class='id'>{id}</div>
      </div>",
                        row.get::<usize, String>(0).unwrap(), // status
                        row.get::<usize, String>(1).unwrap(), // created
                        row.get::<usize, String>(6).unwrap(), // expires
                        row.get::<usize, String>(2).unwrap(), // scheduled
                        row.get::<usize, String>(3).unwrap(), // title
                        row.get::<usize, String>(3).unwrap(), // title
                        row.get::<usize, String>(8).unwrap(), // tags
                        row.get::<usize, String>(8).unwrap(), // tags
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

#[get("/announcements/list/unpublished")]
async fn announcements_list_unpublished(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
) -> impl Responder {
    let mut ret = String::new();

    let mut count = 0;
    loop {
        match pool.get().unwrap().query_row(
            "select * from announcements where status!='published' order by hidden asc limit 1 offset ?",
            [count],
            |row| {
                let id = row.get::<usize, String>(5).unwrap();

                let hide = row.get::<usize, String>(7).unwrap();
                let view_class = match hide.as_str() {
                    "true" => "announcement-hidden",
                    _ => "",
                };

                    ret += format!(
                        "<div class='announcement-{} {view_class}'>
        <div class='date'>
          <div>Created: {}</div>
          <div>Expires: {}</div>
          <div>Scheduled: {}</div>
        </div>
        <div style='display: grid; grid-template-columns: 1fr 1fr'>
          <div class='title' onclick='update_announcement(\"{id}\", \"title\", \"{}\")'>{}</div>
        </div>
        <div class='tags' onclick='update_recurring(\"{id}\", \"tags\", \"{}\"'>{}</div>
        <div class='body' onclick='update_announcement(\"{id}\", \"body\", \"{}\")'>{}</div>
        <button style='color:green' onclick='set_state(\"{id}\", \"approved\")'>Approve</button>
        <button style='color:#770' onclick='update_schedule(\"{id}\")'>Schedule</button>
        <button style='color:orange' onclick='update_expiration(\"{id}\")'>Set Expiration</button>
        <button style='color:red' onclick='set_state(\"{id}\", \"denied\")'>Deny</button>
        <button style='color:maroon' onclick='delete_announcement(\"{id}\")'>Delete</button>
        <button style='color:blue' onclick='hide_announcement(\"{id}\")'>Hide</button>
        <button style='color:blue' onclick='unhide_announcement(\"{id}\")' class='unhide'>Unhide</button>
        <button style='color:purple' onclick='set_state(\"{id}\", \"published\")'>Publish</button>
        <div class='id'>{id}</div>
      </div>",
                        row.get::<usize, String>(0).unwrap(), // status
                        row.get::<usize, String>(1).unwrap(), // created
                        row.get::<usize, String>(6).unwrap(), // expires
                        row.get::<usize, String>(2).unwrap(), // scheduled
                        row.get::<usize, String>(3).unwrap(), // title
                        row.get::<usize, String>(3).unwrap(), // title
                        row.get::<usize, String>(8).unwrap(), // tags
                        row.get::<usize, String>(8).unwrap(), // tags
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
            "select * from announcements order by hidden asc limit 1 offset ?",
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
                    row.get::<usize, String>(7).unwrap(), // tags
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

#[get("/all/")]
async fn all_route(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
) -> impl Responder {
    let mut ret: Vec<Announcement> = Vec::new();

    let mut count = 0;
    loop {
        match pool.get().unwrap().query_row(
            "select * from announcements where status='published' order by hidden asc limit 1 offset ?",
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
                    row.get::<usize, String>(7).unwrap(), // tags
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

    let mut count = 0;
    loop {
        match pool.get().unwrap().query_row(
            "select * from recurring order by hidden asc limit 1 offset ?",
            [count],
            |row| {
                let announcement = Announcement::new(
                    row.get::<usize, String>(1).unwrap(), // title
                    row.get::<usize, String>(2).unwrap(), // body
                    row.get::<usize, String>(3).unwrap(), // created
                    String::new() + "SCHEDULED",
                    row.get::<usize, String>(0).unwrap(), // id
                    String::new() + "STATUS",
                    String::new() + "EXPIRES",
                    String::new() + "MYTAGS",
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
                "false",
                &recurring.tags,
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
                "false",
                &announcement.tags,
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

#[post("/recurring/hide/{id}/{value}")]
async fn recurring_hide(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
    param: web::Path<(String, String)>,
) -> impl Responder {
    let id = param.0.as_str();
    let value = param.1.as_str();

    pool.get()
        .unwrap()
        .execute("update recurring set hidden=?1 where id=?2", (value, id))
        .unwrap();

    "Hide successful"
}

#[post("/announcements/hide/{id}/{value}")]
async fn announcements_hide(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
    param: web::Path<(String, String)>,
) -> impl Responder {
    let id = param.0.as_str();
    let value = param.1.as_str();

    pool.get()
        .unwrap()
        .execute(
            "update announcements set hidden=?1 where id=?2",
            (value, id),
        )
        .unwrap();

    "Hide successful"
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

#[post("/recurring/update/{id}")]
async fn recurring_update(
    pool: web::Data<Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>>,
    id: web::Path<String>,
    change: web::Json<Change>,
) -> impl Responder {
    let sql = format!(
        "update recurring set {} = \"{}\" where id = \"{}\";",
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
