use gtk::glib::{DateTime, GString};

#[derive(Clone)]
pub struct Item {
    pub id: i64,
    pub request: GString,
    pub title: Option<GString>,
    pub opened: Vec<DateTime>,
    pub closed: Vec<DateTime>,
}

impl Item {
    // Constructors

    pub fn create(id: i64, request: GString, title: Option<GString>) -> Self {
        Self {
            id,
            request,
            title,
            opened: vec![now()],
            closed: vec![],
        }
    }

    // Actions

    pub fn open(&mut self) {
        self.opened.push(now())
    }

    pub fn close(&mut self) {
        self.closed.push(now())
    }
}

// Tools

fn now() -> DateTime {
    DateTime::now_local().unwrap()
}
