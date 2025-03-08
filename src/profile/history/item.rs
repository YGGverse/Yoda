use gtk::glib::{DateTime, GString};

#[derive(Clone)]
pub struct Item {
    pub id: i64,
    pub request: GString,
    pub created: DateTime,
    pub opened: DateTime,
    pub closed: Option<DateTime>,
}

impl Item {
    // Constructors

    pub fn create(id: i64, request: GString) -> Self {
        Self {
            id,
            request,
            created: now(),
            opened: now(),
            closed: None,
        }
    }

    // Actions

    pub fn open(&mut self) {
        self.opened = now()
    }

    pub fn close(&mut self) {
        self.closed = Some(now())
    }
}

// Tools

fn now() -> DateTime {
    DateTime::now_local().unwrap()
}
