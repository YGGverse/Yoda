pub mod event;
pub use event::Event;

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
    /// Mark in-memory `Item` as saved
    /// * used for database update (e.g. on app close)
    pub is_saved: bool,
}

impl Item {
    pub fn open(&mut self) {
        self.opened.pulse();
        self.is_saved = false
    }

    pub fn close(&mut self) {
        match self.closed {
            Some(ref mut closed) => closed.pulse(),
            None => self.closed = Some(Event::new()),
        }
        self.is_saved = false
    }
}
