mod common;
mod unsupported;

use adw::AlertDialog;
use common::Common;
use unsupported::Unsupported;

use super::Profile;
use gtk::glib::Uri;
use std::rc::Rc;

/// Create new identity widget for Gemini protocol match given URI
pub fn common(
    profile: &Rc<Profile>,
    request: &Uri,
    callback: &Rc<impl Fn(bool) + 'static>,
) -> Common {
    Common::build(profile, request, callback)
}

/// Create new identity widget for unknown request
pub fn unsupported() -> AlertDialog {
    AlertDialog::unsupported()
}
