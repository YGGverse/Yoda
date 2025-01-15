use gtk::glib::Uri;
use std::cell::Cell;

/// Single redirect `Item`
#[derive(Clone)]
pub struct Item {
    pub is_foreground: bool,
    pub is_processed: Cell<bool>,
    pub referrer: Option<Uri>,
    pub request: Uri,
}
