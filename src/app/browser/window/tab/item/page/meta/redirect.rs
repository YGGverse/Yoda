use gtk::glib::GString;
use std::cell::Cell;

#[derive(Clone, Debug)]
pub struct Redirect {
    pub is_foreground: bool,
    pub is_processed: Cell<bool>,
    pub referrer: Option<GString>,
    pub request: GString,
}
