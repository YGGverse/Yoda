use gtk::glib::{DateTime, GString};

#[derive(Clone)]
pub struct Item {
    /// Queued for DB insert on value is `None` (e.g. on app close)
    pub id: Option<i64>,
    /// The value for navigation request entry
    pub request: GString,
    /// Some history items may contain title (e.g. gemtext documents and system tabs)
    /// * used as the additional criteria for search in the navbar suggestions widget
    pub title: Option<GString>,
    /// Collect opened count with event time
    /// * used for sort order search results in the navbar suggestions widget
    pub opened: Vec<DateTime>,
    /// Collect tab closed count with event time
    /// * used in recently closed pages menu
    pub closed: Vec<DateTime>,
}

impl Item {
    // Constructors

    pub fn init(request: GString, title: Option<GString>) -> Self {
        Self {
            id: None,
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
