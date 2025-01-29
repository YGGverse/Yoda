mod default;
mod unsupported;

use adw::AlertDialog;
use default::Default;
use unsupported::Unsupported;

use super::Profile;
use gtk::glib::Uri;
use std::rc::Rc;

/// Create new identity widget for Gemini protocol match given URI
pub fn default(profile: &Rc<Profile>, request: &Uri, on_apply: impl Fn() + 'static) -> Default {
    Default::build(profile, request, on_apply)
}

/// Create new identity widget for unknown request
pub fn unsupported() -> AlertDialog {
    AlertDialog::unsupported()
}
