use gtk::glib::GString;
use std::cell::Cell;

/// Single redirect `Item`
#[derive(Clone, Debug)]
pub struct Item {
    pub is_foreground: bool,
    pub is_processed: Cell<bool>,
    pub referrer: Option<GString>,
    pub request: GString,
}
