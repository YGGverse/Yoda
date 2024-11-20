mod gemini;
mod unsupported;

use gemini::Gemini;
use unsupported::Unsupported;

use crate::app::browser::window::Action;
use crate::profile::Profile;
use gtk::glib::Uri;
use std::rc::Rc;

/// Create new identity widget for Gemini protocol match given URI
pub fn new_gemini(profile: Rc<Profile>, action: Rc<Action>, auth_uri: Uri) -> Gemini {
    Gemini::new(profile, action, auth_uri)
}

/// Create new identity widget for unknown request
pub fn new_unsupported() -> Unsupported {
    Unsupported::new()
}
