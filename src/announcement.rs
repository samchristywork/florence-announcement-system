use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Announcement {
    pub title: String,
    pub body: String,
    pub created: String,
    pub scheduled: String,
    pub id: String,
    pub status: String,
    pub expires: String,
}

impl Announcement {
    pub fn new(
        title: String,
        body: String,
        created: String,
        scheduled: String,
        id: String,
        status: String,
        expires: String,
    ) -> Self {
        Self {
            title,
            body,
            created,
            scheduled,
            id,
            status,
            expires,
        }
    }
}
