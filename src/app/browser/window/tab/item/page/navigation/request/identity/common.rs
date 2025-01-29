mod widget;
use widget::Widget;

use super::Profile;
use gtk::{glib::Uri, prelude::IsA};
use std::rc::Rc;

pub struct Common {
    // profile: Rc<Profile>,
    widget: Rc<Widget>,
}

impl Common {
    // Construct

    /// Create new `Self` for given `Profile`
    pub fn build(
        profile: &Rc<Profile>,
        request: &Uri,
        callback: &Rc<impl Fn(bool) + 'static>,
    ) -> Self {
        // Init widget
        let widget = Rc::new(Widget::build(profile, request, callback));

        // Return activated `Self`
        Self {
            // profile,
            widget,
        }
    }

    // Actions

    /// Show dialog for parent [Widget](https://docs.gtk.org/gtk4/class.Widget.html)
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.widget.present(parent);
    }
}
