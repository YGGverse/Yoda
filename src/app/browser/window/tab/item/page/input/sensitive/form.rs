mod widget;

use widget::Widget;

use adw::PasswordEntryRow;
use gtk::{gio::SimpleAction, glib::GString};
use std::sync::Arc;

pub struct Form {
    widget: Arc<Widget>,
}

impl Form {
    // Construct
    pub fn new_arc(
        action_send: SimpleAction,
        title: Option<&str>,
        max_length: Option<i32>,
    ) -> Arc<Self> {
        // Init widget
        let widget = Widget::new_arc(action_send, title, max_length);

        // Result
        Arc::new(Self { widget })
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
