mod default;
mod unsupported;

use default::Default;
use unsupported::Unsupported;

use super::{BrowserAction, Profile, WindowAction};
use gtk::glib::Uri;
use std::rc::Rc;

/// Create new identity widget for Gemini protocol match given URI
pub fn default(
    action: (&Rc<BrowserAction>, &Rc<WindowAction>),
    profile: &Rc<Profile>,
    auth_uri: &Uri,
) -> Default {
    Default::build(action, profile, auth_uri)
}

/// Create new identity widget for unknown request
pub fn unsupported() -> Unsupported {
    Unsupported::new()
}
