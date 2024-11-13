mod widget;
use widget::Widget;

use crate::profile::Profile;
use gtk::prelude::IsA;
use std::rc::Rc;

pub struct Welcome {
    profile: Rc<Profile>,
    widget: Rc<Widget>,
}

impl Welcome {
    // Construct

    /// Create new `Self` for given Profile
    pub fn new(profile: Rc<Profile>) -> Self {
        Self {
            profile,
            widget: Rc::new(Widget::new()),
        }
    }

    // Actions

    /// Show dialog for parent [Window](https://docs.gtk.org/gtk4/class.Window.html)
    pub fn present(&self, parent: Option<&impl IsA<gtk::Widget>>) {
        self.widget.present(parent);
    }
}
