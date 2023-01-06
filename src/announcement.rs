use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Announcement {
    pub title: String,
    pub body: String,
    pub created: String,
    pub scheduled: String,
    pub id: String,
    pub status: String,
    pub expires: String,
    pub tags: String,
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
        tags: String,
    ) -> Self {
        Self {
            title,
            body,
            created,
            scheduled,
            id,
            status,
            expires,
            tags,
        }
    }
}
