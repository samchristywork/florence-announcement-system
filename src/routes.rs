use crate::announcement::Announcement;
use actix_web::{get, web, Responder};
use sqlite::State;

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    let mut ret = String::new();

    let connection = sqlite::open("data.sqlite").unwrap();

    let query = "select * from users where age > ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, 50)).unwrap();

    while let Ok(State::Row) = statement.next() {
        ret += format!("name = {}", statement.read::<String, _>("name").unwrap()).as_str();
        ret += format!("age = {}", statement.read::<i64, _>("age").unwrap()).as_str();
    }

    //format!("Hello {name}!")
    ret
}

#[get("/announcements")]
async fn announcements() -> impl Responder {
    let connection = sqlite::open("data.sqlite").unwrap();

    let query = "insert into users values ('Test', 42);";

    connection.execute(query).unwrap();

    let mut ret = String::new();

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
