mod gemini;
mod unsupported;

use gemini::Gemini;
use unsupported::Unsupported;

use crate::app::browser::action::Action as BrowserAction;
use crate::app::browser::window::action::Action as WindowAction;
use crate::profile::Profile;
use gtk::glib::Uri;
use std::rc::Rc;

/// Create new identity widget for Gemini protocol match given URI
pub fn new_gemini(
    action: (Rc<BrowserAction>, Rc<WindowAction>),
    profile: Rc<Profile>,
    auth_uri: Uri,
) -> Gemini {
    Gemini::new(action, profile, auth_uri)
}

/// Create new identity widget for unknown request
pub fn new_unsupported() -> Unsupported {
    Unsupported::new()
}
