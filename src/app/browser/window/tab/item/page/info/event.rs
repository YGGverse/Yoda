// Local dependencies

use gtk::glib::DateTime;

/// Single event holder
/// * used in page info dialog to track page load and parse timings
pub struct Event {
    name: String,
    time: DateTime,
}

impl Event {
    // Constructors

    /// Create new `Self` with auto-completed current local timestamp
    pub fn now(name: String) -> Self {
        Self {
            name,
            time: DateTime::now_local().unwrap(),
        }
    }
}
