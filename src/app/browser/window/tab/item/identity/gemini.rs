mod widget;
use widget::Widget;

use crate::profile::Profile;
use gtk::{glib::Uri, prelude::IsA};
use std::rc::Rc;

pub struct Gemini {
    profile: Rc<Profile>,
    widget: Rc<Widget>,
}

impl Gemini {
    // Construct

    /// Create new `Self` for given Profile
    pub fn new(profile: Rc<Profile>, auth_uri: Uri) -> Self {
        let widget = Rc::new(Widget::new());

        // Init events
        widget.connect_response({
            let profile = profile.clone();
            move |value| {
                match value {
                    Some(id) => {
                        // Activate selected identity ID
                    }
                    None => {
                        // Create and select new identity
                    }
                } // @TODO handle result
            }
        });

        // Return activated `Self`
        Self { profile, widget }
    }

    // Actions

    /// Show dialog for parent [Widget](https://docs.gtk.org/gtk4/class.Widget.html)
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.widget.present(parent);
    }
}
