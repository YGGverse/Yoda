use gtk::glib::{DateTime, GString};

#[derive(Clone)]
pub struct Item {
    pub id: i64,
    pub request: GString,
    pub created: DateTime,
    pub opened: Vec<DateTime>,
    pub closed: Vec<DateTime>,
}

impl Item {
    // Constructors

    pub fn create(id: i64, request: GString) -> Self {
        Self {
            id,
            request,
            created: now(),
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
