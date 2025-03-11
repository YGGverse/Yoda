mod event;

use event::Event;
use gtk::glib::GString;

#[derive(Clone)]
pub struct Item {
    /// Queued for DB insert on value is `None` (e.g. on app close)
    pub id: Option<i64>,
    /// The value for navigation request entry
    pub request: GString,
    /// Some history items may contain title (e.g. gemtext documents and system tabs)
    /// * used as the additional criteria for search in the navbar suggestions widget
    pub title: Option<GString>,
    /// Collect `Item` open events
    /// * used for sort order search results in the navbar suggestions widget and history page
    pub opened: Event,
    /// Collect `Item` close events
    /// * used in recently closed pages menu and history page
    pub closed: Option<Event>,
}

impl Item {
    // Constructors

    pub fn init(request: GString, title: Option<GString>) -> Self {
        Self {
            id: None,
            request,
            title,
            opened: Event::new(),
            closed: None,
        }
    }

    // Actions

    pub fn open(&mut self) {
        self.opened.pulse()
    }

    pub fn close(&mut self) {
        match self.closed {
            Some(ref mut closed) => closed.pulse(),
            None => self.closed = Some(Event::new()),
        }
    }
}
