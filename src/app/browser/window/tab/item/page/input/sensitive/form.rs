mod widget;

use widget::Widget;

use adw::PasswordEntryRow;
use gtk::{gio::SimpleAction, glib::GString};
use std::rc::Rc;

pub struct Form {
    widget: Rc<Widget>,
}

impl Form {
    // Construct
    pub fn new(action_send: SimpleAction, title: Option<&str>, max_length: Option<i32>) -> Self {
        // Init widget
        let widget = Rc::new(Widget::new(action_send, title, max_length));

        // Result
        Self { widget }
    }

    // Actions
    pub fn focus(&self) {
        self.widget.focus();
    }

    // Getters
    pub fn text(&self) -> GString {
        self.widget.text()
    }

    pub fn gobject(&self) -> &PasswordEntryRow {
        self.widget.gobject()
    }
}
