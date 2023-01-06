use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Recurring {
    pub id: String,
    pub title: String,
    pub body: String,
    pub created: String,
    pub mode: String,
    pub time_frame: String,
    pub hidden: String,
    pub tags: String,
}

impl Recurring {
    pub fn new(
        id: String,
        title: String,
        body: String,
        created: String,
        mode: String,
        time_frame: String,
        hidden: String,
        tags: String,
    ) -> Self {
        Self {
            id,
            title,
            body,
            created,
            mode,
            time_frame,
            hidden,
            tags,
        }
    }
}
