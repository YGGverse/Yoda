mod widget;

use widget::Widget;

use gtk::{gio::SimpleAction, glib::GString, TextView};
use std::rc::Rc;

pub struct Form {
    widget: Rc<Widget>,
}

impl Form {
    // Construct
    pub fn new(action_update: SimpleAction) -> Self {
        Self {
            widget: Rc::new(Widget::new(action_update)),
        }
    }

    // Actions
    pub fn focus(&self) {
        self.widget.focus();
    }

    // Getters
    pub fn text(&self) -> GString {
        self.widget.text()
    }

    pub fn gobject(&self) -> &TextView {
        self.widget.gobject()
    }
}
