pub struct Announcement<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub created: &'a str,
    pub scheduled: &'a str,
    pub id: &'a str,
    pub status: &'a str,
}

impl<'a> Announcement<'a> {
    pub fn new(
        title: &'a str,
        body: &'a str,
        created: &'a str,
        scheduled: &'a str,
        id: &'a str,
        status: &'a str,
    ) -> Self {
        Self {
            title,
            body,
            created,
            scheduled,
            id,
            status,
        }
    }
}
